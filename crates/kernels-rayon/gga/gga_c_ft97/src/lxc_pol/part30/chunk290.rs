//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 290/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk290(t3938: f64, t734: f64, t91: f64, t3688: f64, t3710: f64, t2339: f64, t2342: f64, t2533: f64, t3693: f64, t3697: f64, t3702: f64, t3707: f64, t3715: f64, t3720: f64, t3824: f64, t3904: f64) -> (f64, f64) {
    let t3940 = t91 * t734 * t3938;
    let t3942 = t3688 / 27.0_f64;
    let t3947 = t3710 / 9.0_f64;
    let t3951 = -t3904 / 12.0_f64 + t3940 / 6.0_f64 + t2533 + t2339 + t2342 + t3942 - 2.0_f64 / 27.0_f64 * t3693 + t3697 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t3702 + 2.0_f64 / 9.0_f64 * t3707 + t3947 + t3715 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t3720 - t3824 / 3.0_f64;
    (t3940, t3951)
}
