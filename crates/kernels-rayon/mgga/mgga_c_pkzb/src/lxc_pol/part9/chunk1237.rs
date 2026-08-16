//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1237/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1237(t17867: f64, t2104: f64, t2911: f64, t2064: f64, t2922: f64, t2924: f64, t5974: f64, t7672: f64, t21604: f64, t21607: f64, t21611: f64, t21614: f64, t21617: f64, t21620: f64, t7683: f64, t7707: f64, t7725: f64, t7729: f64, t7733: f64, t7739: f64, t7745: f64, t7776: f64) -> f64 {
    let t21623 = t2104 * t17867 * t2911;
    let t21624 = 0.28582678745379824648e-3_f64 * t21623;
    let t21626 = t2922 * t2064 * t2924;
    let t21627 = 0.14291339372689912324e-3_f64 * t21626;
    let t21633 = t2104 * t5974 * t7672;
    let t21635 = -0.13719685797782315831e-1_f64 * t7707 * t7729 - 0.68598428988911579154e-2_f64 * t7707 * t7733 - 0.20579528696673473747e-1_f64 * t21604 * t7739 + 0.20579528696673473747e-1_f64 * t21607 * t7745 + 0.17149607247227894789e-2_f64 * t21611 + 0.85748036236139473944e-3_f64 * t21614 + 0.25724410870841842184e-2_f64 * t21617 - 0.25724410870841842184e-2_f64 * t21620 + t21624 + t21627 + 0.13719685797782315831e-1_f64 * t7707 * t7683 - 0.68598428988911579154e-2_f64 * t7725 * t7776 - 0.17149607247227894789e-2_f64 * t21633;
    t21635
}
