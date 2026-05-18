//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1234/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1234<F: Float>(t218: F, t219: F, t2739: F, t3515: F, t10821: F, t675: F, t10825: F, t17548: F, t20748: F, t20751: F, t20754: F, t20861: F, t30314: F, t30316: F, t30319: F, t30322: F, t30324: F, t30326: F, t30328: F, t30331: F, t30338: F, t30342: F, t30346: F) -> (F, F, F, F) {
    let t30350 = t218 * t219 * t2739 * t3515;
    let t30353 = t218 * t675 * t10821;
    let t30356 = t218 * t675 * t10825;
    let t30358 = F::new(0.46074375e0) * t30314 + F::new(0.46074375e0) * t30316 + F::new(0.15358125e0) * t30319 - F::new(0.3560484375e1) * t30322 + F::new(0.427258125e1) * t30324 - F::new(0.28483875e1) * t30326 - F::new(0.28483875e1) * t30328 - F::new(0.9494625e0) * t30331 + t20861 + F::new(0.82156666666666666666e0) * t20748 + F::new(0.82156666666666666666e0) * t20751 - F::new(0.21908444444444444444e1) * t20754 + t17548 + F::new(0.73941e0) * t30338 + F::new(0.24647e0) * t30342 + F::new(0.24647e0) * t30346 + F::new(0.73941e0) * t30350 - F::new(0.49294e0) * t30353 - F::new(0.16431333333333333333e0) * t30356;
    (t30350, t30353, t30356, t30358)
}
