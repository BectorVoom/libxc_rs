//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 816/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk816<F: Float>(t1476: F, t4932: F, t1475: F, t475: F, t574: F, t474: F, t49: F, t4902: F, t55: F, t47: F, t82: F, t1489: F, t482: F, t50: F, t65: F, t4929: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4933 = t1476 * t4932;
    let t4934 = t1475 * t4933;
    let t4936 = t475 * t574;
    let t4937 = t474 * t4936;
    let t4939 = t49 * t4902;
    let t4941 = 1.0/pow_3_2(t55);
    let t4942 = t4941 * t47;
    let t4943 = t4942 * t82;
    let t4945 = t1489 * t4933;
    let t4947 = t482 * t4936;
    let t4950 = t65 * t50 * t4932;
    let t4952 = -0.34523333333333333333e1 * t4929 + 0.23015555555555555556e1 * t4934 - 0.26851481481481481482e1 * t4937 - 0.93932222222222222223e0 * t4939 + 0.73355e-1 * t4943 - 0.14671e0 * t4945 - 0.17116166666666666667e0 * t4947 - 0.36793333333333333333e0 * t4950;
    (t4934, t4937, t4939, t4942, t4943, t4945, t4947, t4950, t4952)
}
