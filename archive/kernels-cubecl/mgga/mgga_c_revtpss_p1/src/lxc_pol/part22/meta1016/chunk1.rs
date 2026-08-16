//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3509/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3509<F: Float>(t11922: F, t15906: F, t19753: F, t1011: F, t1012: F, t1015: F, t11687: F, t15689: F, t15691: F, t19501: F, t19982: F, t19993: F, t19998: F, t23898: F, t3117: F, t42804: F, t43050: F, t43051: F, t54014: F, t54036: F, t54039: F, t54042: F, t54047: F, t54166: F, t54801: F, t55137: F, t60754: F, t6266: F) -> F {
    let t66288 = t15906 * t11922 * t19753;
    let t66294 = -F::cast_from(0.20325460441158986416e-2_f64) * t54014 - F::cast_from(0.3811023832717309953e-3_f64) * t54036 - F::cast_from(0.28582678745379824648e-3_f64) * t15689 * t15691 * t11687 * t6266 - F::cast_from(0.17149607247227894789e-2_f64) * t54801 * t15691 * t42804 * t23898 - F::cast_from(0.3811023832717309953e-3_f64) * t54039 - F::cast_from(0.19055119163586549765e-3_f64) * t54042 - F::cast_from(0.31758531939310916275e-3_f64) * t54047 - F::cast_from(0.11433071498151929859e-2_f64) * t54166 * t19993 + F::cast_from(0.11433071498151929859e-2_f64) * t55137 * t19998 + F::cast_from(0.95275595817932748826e-3_f64) * t54166 * t19982 + t1011 * t1012 * t1015 * t60754 / F::cast_from(288.0_f64) - F::cast_from(0.1714960724722789479e-2_f64) * t66288 + F::cast_from(0.85748036236139473944e-3_f64) * t43050 * t3117 * t19501 * t43051;
    t66294
}
