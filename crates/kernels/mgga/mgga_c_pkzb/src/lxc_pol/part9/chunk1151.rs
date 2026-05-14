//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1151/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1151<F: Float>(t6269: F, t8016: F, t898: F, t2332: F, t8028: F, t3152: F, t6279: F, t2328: F, t8303: F, t3147: F, t6124: F, t2295: F, t8098: F, t891: F, t237: F, t6282: F) -> (F, F, F, F, F, F, F) {
    let t22162 = 0.14035736694323150897e2 * t898 * t8016 * t6269;
    let t22164 = 0.35089341735807877242e1 * t8028 * t2332;
    let t22167 = 0.11696447245269292414e1 * t898 * t3152 * t6279;
    let t22169 = 0.10526802520742363173e2 * t2328 * t8303;
    let t22171 = 0.10389515463408878255e3 * t3147 * t6124;
    let t22175 = 0.35089341735807877242e1 * t898 * t2295 * t8098 * t891;
    let t22180 = t237 * t6282;
    (t22162, t22164, t22167, t22169, t22171, t22175, t22180)
}
