//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 891/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk891(t13811: f64, t13361: f64, t2354: f64, t446: f64, t13346: f64, t724: f64, t13352: f64, t2594: f64, t13356: f64, t9770: f64, t13798: f64, t13801: f64, t13804: f64, t13807: f64, t13810: f64, t9972: f64) -> (f64, f64, f64, f64, f64) {
    let t13812 = 4.0_f64 / 27.0_f64 * t13811;
    let t13813 = t2354 * t13361;
    let t13814 = t446 * t13813;
    let t13816 = t724 * t13346;
    let t13817 = t446 * t13816;
    let t13819 = t2594 * t13352;
    let t13820 = t446 * t13819;
    let t13822 = t9770 * t13356;
    let t13823 = t446 * t13822;
    let t13825 = -2.0_f64 / 27.0_f64 * t13798 - 10.0_f64 / 81.0_f64 * t13801 + 8.0_f64 / 27.0_f64 * t13804 + t13807 / 9.0_f64 - t13810 - t9972 - t13812 - 2.0_f64 / 9.0_f64 * t13814 - 2.0_f64 / 3.0_f64 * t13817 + 4.0_f64 / 9.0_f64 * t13820 - 2.0_f64 / 9.0_f64 * t13823;
    (t13814, t13817, t13820, t13823, t13825)
}
