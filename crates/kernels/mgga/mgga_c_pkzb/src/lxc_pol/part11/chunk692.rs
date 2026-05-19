//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 692/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk692<F: Float>(t1476: F, t4932: F, t1475: F, t475: F, t574: F, t474: F, t49: F, t4902: F, t55: F, t47: F, t82: F, t1489: F) -> (F, F, F, F, F, F, F) {
    let t4933 = t1476 * t4932;
    let t4934 = t1475 * t4933;
    let t4936 = t475 * t574;
    let t4937 = t474 * t4936;
    let t4939 = t49 * t4902;
    let t4941 = F::new(1.0)/pow_3_2::<F>(t55);
    let t4942 = t4941 * t47;
    let t4943 = t4942 * t82;
    let t4945 = t1489 * t4933;
    (t4934, t4936, t4937, t4939, t4942, t4943, t4945)
}
