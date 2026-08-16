//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1164/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1164(t1353: f64, t7933: f64, t1907: f64, t7311: f64, t120967: f64, t1399: f64, t1868: f64, t247: f64, t561: f64, t120962: f64, t32284: f64, t5705: f64) -> (f64, f64, f64, f64) {
    let t125559 = t7933 * t1353;
    let t125563 = t1907 * t7311;
    let t125570 = t120967 * t247 * t561 * t1868 * t1399;
    let t125573 = t32284 * t120962 * t5705;
    (t125559, t125563, t125570, t125573)
}
