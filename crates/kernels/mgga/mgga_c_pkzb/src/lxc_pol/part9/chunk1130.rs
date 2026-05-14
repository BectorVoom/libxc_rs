//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1130/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1130<F: Float>(t21626: F, t2104: F, t5974: F, t7672: F, t21604: F, t21607: F, t21611: F, t21614: F, t21617: F, t21620: F, t21624: F, t7683: F, t7707: F, t7725: F, t7729: F, t7733: F, t7739: F, t7745: F, t7776: F) -> (F,) {
    let t21627 = 0.14291339372689912324e-3 * t21626;
    let t21633 = t2104 * t5974 * t7672;
    let t21635 = -0.13719685797782315831e-1 * t7707 * t7729 - 0.68598428988911579154e-2 * t7707 * t7733 - 0.20579528696673473747e-1 * t21604 * t7739 + 0.20579528696673473747e-1 * t21607 * t7745 + 0.17149607247227894789e-2 * t21611 + 0.85748036236139473944e-3 * t21614 + 0.25724410870841842184e-2 * t21617 - 0.25724410870841842184e-2 * t21620 + t21624 + t21627 + 0.13719685797782315831e-1 * t7707 * t7683 - 0.68598428988911579154e-2 * t7725 * t7776 - 0.17149607247227894789e-2 * t21633;
    (t21635,)
}
