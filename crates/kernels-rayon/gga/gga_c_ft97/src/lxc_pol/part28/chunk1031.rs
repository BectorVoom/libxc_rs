//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1031/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1031(t136116: f64, t28: f64, t89: f64, t942: f64, t136241: f64, t136243: f64, t136250: f64, t137070: f64, t137073: f64, t137102: f64, t137623: f64, t144946: f64, t144950: f64, t144953: f64, t144956: f64, t144961: f64, t144966: f64, t144970: f64, t144974: f64) -> (f64, f64) {
    let t144978 = t89 * t28 * t136116 * t942;
    let t144981 = -4.0_f64 / 9.0_f64 * t136241 - t144946 / 12.0_f64 + t136243 / 9.0_f64 - 8.0_f64 / 3.0_f64 * t144950 - 2.0_f64 / 3.0_f64 * t144953 + 2.0_f64 / 9.0_f64 * t144956 + t144961 - 2.0_f64 / 3.0_f64 * t136250 + 2.0_f64 * t137070 - 4.0_f64 / 3.0_f64 * t137073 - 8.0_f64 / 3.0_f64 * t144966 + 2.0_f64 * t144970 + 2.0_f64 * t144974 + 2.0_f64 * t144978 - t137623 - t137102 / 12.0_f64;
    (t144978, t144981)
}
