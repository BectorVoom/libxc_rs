//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1237/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1237<F: Float>(t17867: F, t2104: F, t2911: F, t2064: F, t2922: F, t2924: F, t5974: F, t7672: F, t21604: F, t21607: F, t21611: F, t21614: F, t21617: F, t21620: F, t7683: F, t7707: F, t7725: F, t7729: F, t7733: F, t7739: F, t7745: F, t7776: F) -> F {
    let t21623 = t2104 * t17867 * t2911;
    let t21624 = F::cast_from(0.28582678745379824648e-3_f64) * t21623;
    let t21626 = t2922 * t2064 * t2924;
    let t21627 = F::cast_from(0.14291339372689912324e-3_f64) * t21626;
    let t21633 = t2104 * t5974 * t7672;
    let t21635 = -F::cast_from(0.13719685797782315831e-1_f64) * t7707 * t7729 - F::cast_from(0.68598428988911579154e-2_f64) * t7707 * t7733 - F::cast_from(0.20579528696673473747e-1_f64) * t21604 * t7739 + F::cast_from(0.20579528696673473747e-1_f64) * t21607 * t7745 + F::cast_from(0.17149607247227894789e-2_f64) * t21611 + F::cast_from(0.85748036236139473944e-3_f64) * t21614 + F::cast_from(0.25724410870841842184e-2_f64) * t21617 - F::cast_from(0.25724410870841842184e-2_f64) * t21620 + t21624 + t21627 + F::cast_from(0.13719685797782315831e-1_f64) * t7707 * t7683 - F::cast_from(0.68598428988911579154e-2_f64) * t7725 * t7776 - F::cast_from(0.17149607247227894789e-2_f64) * t21633;
    t21635
}
