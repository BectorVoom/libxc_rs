//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 616/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk616(t25912: f64, t89: f64, t22873: f64, t942: f64, t28: f64, t3204: f64, t5691: f64, t22958: f64, t5674: f64, t25873: f64, t25876: f64, t25881: f64, t25886: f64, t25891: f64, t25897: f64, t25902: f64, t25906: f64, t25910: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25913 = t89 * t25912;
    let t25915 = t22873 * t942;
    let t25916 = t28 * t25915;
    let t25917 = t89 * t25916;
    let t25919 = t5691 * t3204;
    let t25920 = t22958 * t25919;
    let t25921 = t5674 * t25920;
    let t25923 = -t25873 + t25876 / 18.0_f64 + t25881 / 9.0_f64 - t25886 / 6.0_f64 - t25891 / 6.0_f64 - t25897 / 8.0_f64 + t25902 / 18.0_f64 + 2.0_f64 / 3.0_f64 * t25906 + 2.0_f64 / 3.0_f64 * t25910 - 2.0_f64 / 9.0_f64 * t25913 + 2.0_f64 / 3.0_f64 * t25917 - t25921 / 9.0_f64;
    (t25913, t25916, t25917, t25919, t25921, t25923)
}
