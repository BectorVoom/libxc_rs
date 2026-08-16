//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 505/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk505(t53: f64, t50: f64, t983: f64, t1375: f64, t1378: f64, t154: f64, t437: f64, t5328: f64, t5498: f64, t814: f64, t913: f64, t916: f64, t4408: f64, t525: f64, zeta_threshold: f64) -> (f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t5501 = t983 * t50;
    let t5511 = piecewise3(t54, 0.0_f64, 8.0_f64 / 27.0_f64 * t5498 * t913 - 8.0_f64 / 9.0_f64 * t5501 * t5328 - 2.0_f64 / 9.0_f64 * t1375 * t916 + 4.0_f64 / 3.0_f64 * t437 * t814 - 4.0_f64 * t1378 * t154);
    let t5512 = t4408 * t525;
    (t5511, t5512)
}
