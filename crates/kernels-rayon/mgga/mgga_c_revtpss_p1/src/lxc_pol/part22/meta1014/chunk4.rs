//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3495/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3495(t1062: f64, t53877: f64, t15827: f64, t19878: f64, t1042: f64, t1063: f64, t11656: f64, t15592: f64, t15618: f64, t15670: f64, t15719: f64, t15725: f64, t15935: f64, t16183: f64, t19702: f64, t19944: f64, t3105: f64, t3117: f64, t357: f64, t42324: f64, t42326: f64, t4788: f64, t4839: f64, t4893: f64, t4899: f64, t53567: f64, t53612: f64, t54471: f64, t65823: f64, t65829: f64) -> f64 {
    let t65837 = t53877 * t1062;
    let t65840 = t19878 * t15827;
    let t65852 = 0.3811023832717309953e-3_f64 * t53567 - 0.30488190661738479624e-2_f64 * t54471 * t4788 + 0.3811023832717309953e-3_f64 * t65823 + 0.28582678745379824648e-3_f64 * t15618 * t15592 + 0.15244095330869239812e-2_f64 * t11656 * t19702 + 0.85748036236139473944e-3_f64 * t1063 * t1042 * t15935 * t65829 - 0.91464571985215438873e-2_f64 * t15670 * t3105 * t4839 - 0.25724410870841842183e-2_f64 * t65837 * t15719 + 0.11433071498151929859e-2_f64 * t65840 + 0.17149607247227894789e-2_f64 * t15725 * t19944 + 0.5081365110289746604e-3_f64 * t42324 + 0.1270341277572436651e-3_f64 * t42326 + 0.19055119163586549765e-3_f64 * t53612 - 0.42874018118069736972e-3_f64 * t4899 * t3117 * t4893 * t357 * t16183;
    t65852
}
