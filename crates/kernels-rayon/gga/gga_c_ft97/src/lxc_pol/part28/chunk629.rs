//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 629/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk629(t26061: f64, t492: f64, t25873: f64, t25876: f64, t25881: f64, t25886: f64, t25891: f64, t25897: f64, t25902: f64, t25906: f64, t25910: f64, t25913: f64, t25917: f64, t25921: f64) -> (f64, f64) {
    let t26062 = t26061 * t492;
    let t26077 = -3.0_f64 * t25873 + t25876 / 6.0_f64 + t25881 / 3.0_f64 - t25886 / 2.0_f64 - t25891 / 2.0_f64 - 3.0_f64 / 8.0_f64 * t25897 + t25902 / 6.0_f64 + 2.0_f64 * t25906 + 2.0_f64 * t25910 - 2.0_f64 / 3.0_f64 * t25913 + 2.0_f64 * t25917 - t25921 / 3.0_f64;
    (t26062, t26077)
}
