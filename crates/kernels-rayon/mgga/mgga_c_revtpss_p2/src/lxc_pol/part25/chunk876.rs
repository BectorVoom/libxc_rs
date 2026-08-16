//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 876/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk876(t10345: f64, t10357: f64, t10361: f64, t10364: f64, t10369: f64, t10373: f64, t10376: f64, t10379: f64, t2270: f64, t2276: f64, t2279: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64) -> f64 {
    let t10380 = -1232.0_f64 / 27.0_f64 * t10345 * t49 + 220.0_f64 / 9.0_f64 * t2270 * t617 - 20.0_f64 / 9.0_f64 * t614 * t2276 - 20.0_f64 / 3.0_f64 * t614 * t2279 - 5.0_f64 / 108.0_f64 * t44 * t10357 + 5.0_f64 / 6.0_f64 * t44 * t10361 + 5.0_f64 / 6.0_f64 * t44 * t10364 + 5.0_f64 / 108.0_f64 * t56 * t10369 + 5.0_f64 / 6.0_f64 * t56 * t10373 - 5.0_f64 / 6.0_f64 * t56 * t10376 + t10379;
    t10380
}
