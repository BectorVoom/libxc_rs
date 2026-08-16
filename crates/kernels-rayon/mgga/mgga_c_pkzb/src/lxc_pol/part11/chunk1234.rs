//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1234/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1234(t218: f64, t219: f64, t2739: f64, t3515: f64, t10821: f64, t675: f64, t10825: f64, t17548: f64, t20748: f64, t20751: f64, t20754: f64, t20861: f64, t30314: f64, t30316: f64, t30319: f64, t30322: f64, t30324: f64, t30326: f64, t30328: f64, t30331: f64, t30338: f64, t30342: f64, t30346: f64) -> (f64, f64, f64, f64) {
    let t30350 = t218 * t219 * t2739 * t3515;
    let t30353 = t218 * t675 * t10821;
    let t30356 = t218 * t675 * t10825;
    let t30358 = 0.46074375e0_f64 * t30314 + 0.46074375e0_f64 * t30316 + 0.15358125e0_f64 * t30319 - 0.3560484375e1_f64 * t30322 + 0.427258125e1_f64 * t30324 - 0.28483875e1_f64 * t30326 - 0.28483875e1_f64 * t30328 - 0.9494625e0_f64 * t30331 + t20861 + 0.82156666666666666666e0_f64 * t20748 + 0.82156666666666666666e0_f64 * t20751 - 0.21908444444444444444e1_f64 * t20754 + t17548 + 0.73941e0_f64 * t30338 + 0.24647e0_f64 * t30342 + 0.24647e0_f64 * t30346 + 0.73941e0_f64 * t30350 - 0.49294e0_f64 * t30353 - 0.16431333333333333333e0_f64 * t30356;
    (t30350, t30353, t30356, t30358)
}
