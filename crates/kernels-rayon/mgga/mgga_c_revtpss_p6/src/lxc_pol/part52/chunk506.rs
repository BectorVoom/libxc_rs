//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 506/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk506(t1340: f64, t2496: f64, t1330: f64, t177: f64, t762: f64, t2626: f64, t1389: f64, t1408: f64, t2736: f64, t1419: f64, t213: f64, t1425: f64, t560: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4037 = 0.17315859105681463759e2_f64 * t1340 * t2496;
    let t4038 = t1330 * t177;
    let t4039 = t4038 * t762;
    let t4042 = 0.11696447245269292414e1_f64 * t1340 * t2626;
    let t4062 = t1408 * t1389;
    let t4064 = 0.25410001404642664112e-5_f64 * t2736 * t4062;
    let t4071 = t213 * t1419;
    let t4075 = 1.0_f64 / t1425 / t560;
    (t4037, t4039, t4042, t4064, t4071, t4075)
}
