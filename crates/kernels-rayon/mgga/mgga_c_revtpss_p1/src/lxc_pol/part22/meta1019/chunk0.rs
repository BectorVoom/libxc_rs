//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3531/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3531(t11859: f64, t11922: f64, t19635: f64, t1043: f64, t19971: f64, t11875: f64, t19640: f64, t11675: f64, t15622: f64, t15906: f64, t15907: f64, t15963: f64, t19611: f64, t19778: f64, t19873: f64, t20096: f64, t3091: f64, t3092: f64, t3117: f64, t42417: f64, t42996: f64, t53885: f64, t54500: f64, t54785: f64, t54792: f64, t54795: f64, t6268: f64) -> (f64, f64) {
    let t66943 = t11859 * t11922 * t19635;
    let t66945 = t19971 * t1043;
    let t66951 = t11875 * t11922 * t19640;
    let t66956 = 0.17149607247227894789e-2_f64 * t54500 * t15622 - 0.60976381323476959249e-2_f64 * t53885 * t20096 + 0.96545937095505185476e-2_f64 * t42417 * t6268 - 0.28582678745379824648e-3_f64 * t3091 * t3092 * t19611 * t15963 - 0.57165357490759649296e-3_f64 * t11675 * t19873 + 0.1270341277572436651e-3_f64 * t42996 + 0.57165357490759649296e-3_f64 * t11675 * t19778 - 0.11433071498151929859e-2_f64 * t66943 - 0.51448821741683684367e-2_f64 * t15906 * t3117 * t15907 * t66945 + 0.57165357490759649296e-3_f64 * t66951 + 0.19055119163586549765e-3_f64 * t54785 - t54792 / 81.0_f64 + t54795 / 432.0_f64;
    (t66945, t66956)
}
