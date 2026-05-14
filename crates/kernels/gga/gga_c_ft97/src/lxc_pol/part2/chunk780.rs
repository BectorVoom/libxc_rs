//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 780/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk780<F: Float>(t13819: F, t446: F, t13356: F, t9770: F, t13798: F, t13801: F, t13804: F, t13807: F, t13810: F, t13812: F, t13814: F, t13817: F, t9972: F, t13724: F, t13761: F, t13796: F) -> (F, F, F) {
    let t13820 = t446 * t13819;
    let t13822 = t9770 * t13356;
    let t13823 = t446 * t13822;
    let t13825 = -2.0 / 27.0 * t13798 - 10.0 / 81.0 * t13801 + 8.0 / 27.0 * t13804 + t13807 / 9.0 - t13810 - t9972 - t13812 - 2.0 / 9.0 * t13814 - 2.0 / 3.0 * t13817 + 4.0 / 9.0 * t13820 - 2.0 / 9.0 * t13823;
    let t13827 = t13724 + t13761 + t13796 + t13825;
    (t13820, t13823, t13827)
}
