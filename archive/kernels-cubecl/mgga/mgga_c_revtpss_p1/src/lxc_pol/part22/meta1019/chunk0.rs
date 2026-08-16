//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3531/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3531<F: Float>(t11859: F, t11922: F, t19635: F, t1043: F, t19971: F, t11875: F, t19640: F, t11675: F, t15622: F, t15906: F, t15907: F, t15963: F, t19611: F, t19778: F, t19873: F, t20096: F, t3091: F, t3092: F, t3117: F, t42417: F, t42996: F, t53885: F, t54500: F, t54785: F, t54792: F, t54795: F, t6268: F) -> (F, F) {
    let t66943 = t11859 * t11922 * t19635;
    let t66945 = t19971 * t1043;
    let t66951 = t11875 * t11922 * t19640;
    let t66956 = F::cast_from(0.17149607247227894789e-2_f64) * t54500 * t15622 - F::cast_from(0.60976381323476959249e-2_f64) * t53885 * t20096 + F::cast_from(0.96545937095505185476e-2_f64) * t42417 * t6268 - F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t3092 * t19611 * t15963 - F::cast_from(0.57165357490759649296e-3_f64) * t11675 * t19873 + F::cast_from(0.1270341277572436651e-3_f64) * t42996 + F::cast_from(0.57165357490759649296e-3_f64) * t11675 * t19778 - F::cast_from(0.11433071498151929859e-2_f64) * t66943 - F::cast_from(0.51448821741683684367e-2_f64) * t15906 * t3117 * t15907 * t66945 + F::cast_from(0.57165357490759649296e-3_f64) * t66951 + F::cast_from(0.19055119163586549765e-3_f64) * t54785 - t54792 / F::cast_from(81.0_f64) + t54795 / F::cast_from(432.0_f64);
    (t66945, t66956)
}
