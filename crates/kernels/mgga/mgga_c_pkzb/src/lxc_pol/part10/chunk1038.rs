//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1038/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1038<F: Float>(t7012: F, t114: F, t3380: F, t557: F, t5078: F, t5080: F, t126: F, t8748: F, t83: F, t545: F, t3501: F, t5165: F, t1536: F, t3401: F, t5149: F, t1020: F, t1535: F, t2536: F, t2718: F, t3396: F, t5091: F, t5130: F, t5139: F, t5141: F, t5148: F, t637: F, t7015: F, t7017: F, t7019: F, t7022: F, t7201: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8769 = 16.0 * t7012;
    let t8770 = t3380 * t114;
    let t8771 = t8770 * t557;
    let t8772 = 0.5848223622634646207e0 * t8771;
    let t8773 = 8.0 * t5078;
    let t8774 = 8.0 * t5080;
    let t8775 = t8748 * t126;
    let t8776 = t83 * t8775;
    let t8777 = t3380 * t545;
    let t8778 = t83 * t8777;
    let t8779 = t3501 * t5165;
    let t8783 = t1536 * t3401;
    let t8789 = 0.11696447245269292414e1 * t5149;
    let t8793 = 6.0 * t1020 * t1535 * t7201 + 3.0 * t1535 * t1536 * t3396 + 2.0 * t2536 * t637 * t8779 + 6.0 * t2718 * t8783 + t5091 - t5130 - t5139 + t5141 - t5148 - t7015 - t7017 + t7019 + t7022 - t8769 - t8772 - t8773 - t8774 + t8776 + t8778 + t8789;
    (t8769, t8770, t8772, t8773, t8774, t8775, t8776, t8777, t8778, t8779, t8783, t8789, t8793)
}
