//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 891/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk891<F: Float>(t13811: F, t13361: F, t2354: F, t446: F, t13346: F, t724: F, t13352: F, t2594: F, t13356: F, t9770: F, t13798: F, t13801: F, t13804: F, t13807: F, t13810: F, t9972: F) -> (F, F, F, F, F) {
    let t13812 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13811;
    let t13813 = t2354 * t13361;
    let t13814 = t446 * t13813;
    let t13816 = t724 * t13346;
    let t13817 = t446 * t13816;
    let t13819 = t2594 * t13352;
    let t13820 = t446 * t13819;
    let t13822 = t9770 * t13356;
    let t13823 = t446 * t13822;
    let t13825 = -F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t13798 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t13801 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13804 + t13807 / F::cast_from(9.0_f64) - t13810 - t9972 - t13812 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13814 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13817 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13820 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13823;
    (t13814, t13817, t13820, t13823, t13825)
}
