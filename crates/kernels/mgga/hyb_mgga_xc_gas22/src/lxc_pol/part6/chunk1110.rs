//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1110/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1110<F: Float>(t21506: F, t2476: F, t2519: F, t347: F, t21389: F, t2470: F, t2520: F, t21424: F, t2575: F, t2597: F, t2569: F, t2576: F, t7108: F, t993: F, t7058: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21507 = 1.0 / t21506;
    let t21537 = t347 / t2519 / t2476;
    let t21541 = 0.96141975308641975307e-1 * t21389;
    let t21552 = t2470 * t2520;
    let t21557 = 0.31003950617283950618e1 * t21389;
    let t21560 = 0.13388493827160493828e1 * t21424;
    let t21587 = 0.18467901234567901234e0 * t21389;
    let t21601 = 1.0 / t2597 / t2575;
    let t21613 = t2569 * t2576;
    let t21616 = t993 * t7108;
    let t21620 = 0.17757530864197530864e0 * t21389;
    let t21628 = t974 * t7058;
    (t21507, t21537, t21541, t21552, t21557, t21560, t21587, t21601, t21613, t21616, t21620, t21628)
}
