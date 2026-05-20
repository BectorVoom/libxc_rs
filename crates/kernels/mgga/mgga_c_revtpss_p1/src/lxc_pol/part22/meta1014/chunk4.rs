//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3495/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3495<F: Float>(t1062: F, t53877: F, t15827: F, t19878: F, t1042: F, t1063: F, t11656: F, t15592: F, t15618: F, t15670: F, t15719: F, t15725: F, t15935: F, t16183: F, t19702: F, t19944: F, t3105: F, t3117: F, t357: F, t42324: F, t42326: F, t4788: F, t4839: F, t4893: F, t4899: F, t53567: F, t53612: F, t54471: F, t65823: F, t65829: F) -> F {
    let t65837 = t53877 * t1062;
    let t65840 = t19878 * t15827;
    let t65852 = F::cast_from(0.3811023832717309953e-3_f64) * t53567 - F::cast_from(0.30488190661738479624e-2_f64) * t54471 * t4788 + F::cast_from(0.3811023832717309953e-3_f64) * t65823 + F::cast_from(0.28582678745379824648e-3_f64) * t15618 * t15592 + F::cast_from(0.15244095330869239812e-2_f64) * t11656 * t19702 + F::cast_from(0.85748036236139473944e-3_f64) * t1063 * t1042 * t15935 * t65829 - F::cast_from(0.91464571985215438873e-2_f64) * t15670 * t3105 * t4839 - F::cast_from(0.25724410870841842183e-2_f64) * t65837 * t15719 + F::cast_from(0.11433071498151929859e-2_f64) * t65840 + F::cast_from(0.17149607247227894789e-2_f64) * t15725 * t19944 + F::cast_from(0.5081365110289746604e-3_f64) * t42324 + F::cast_from(0.1270341277572436651e-3_f64) * t42326 + F::cast_from(0.19055119163586549765e-3_f64) * t53612 - F::cast_from(0.42874018118069736972e-3_f64) * t4899 * t3117 * t4893 * t357 * t16183;
    t65852
}
