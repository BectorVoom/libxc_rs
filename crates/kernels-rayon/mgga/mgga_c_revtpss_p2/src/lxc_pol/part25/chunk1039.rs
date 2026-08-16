//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1039/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1039(t12292: f64, t12296: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t1132: f64) -> (f64, f64) {
    let t12322 = -t12296 + 4.0_f64 / 9.0_f64 * t12297 + 2.0_f64 / 9.0_f64 * t12299 - 2.0_f64 / 3.0_f64 * t12301 - t12303 / 3.0_f64 + 10.0_f64 / 27.0_f64 * t12307 - 4.0_f64 / 3.0_f64 * t12310 - 2.0_f64 / 3.0_f64 * t12292 + 2.0_f64 * t12314 + 2.0_f64 * t12317 + t12320 / 3.0_f64;
    let t12323 = t1132 * t12322;
    (t12322, t12323)
}
