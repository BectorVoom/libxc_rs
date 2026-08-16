//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3488/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3488<F: Float>(t16163: F, t4879: F, t1063: F, t19681: F, t3172: F, t11710: F, t19625: F, t4899: F, t19687: F, t3160: F, t65338: F, t11672: F, t11675: F, t11994: F, t15963: F, t1671: F, t19501: F, t19682: F, t19702: F, t19778: F, t19782: F, t3092: F, t3164: F, t3188: F, t42391: F, t4783: F, t54144: F, t54471: F, t6263: F) -> F {
    let t65627 = t4879 * t16163;
    let t65630 = t1063 * t3172 * t19681;
    let t65637 = t4899 * t11710 * t19625;
    let t65650 = t1063 * t3172 * t19687;
    let t65654 = t65338 * t3160;
    let t65659 = F::cast_from(0.57165357490759649296e-3_f64) * t65627 - F::cast_from(0.3811023832717309953e-3_f64) * t65630 - F::cast_from(0.30488190661738479624e-2_f64) * t54471 * t4783 - F::cast_from(0.30488190661738479624e-2_f64) * t11672 * t19778 - F::cast_from(0.19055119163586549765e-3_f64) * t65637 + F::cast_from(0.28582678745379824648e-3_f64) * t4899 * t3092 * t19501 * t15963 + F::cast_from(0.47637797908966374414e-3_f64) * t11675 * t19782 + F::cast_from(0.14481890564325777821e-1_f64) * t54144 * t1671 - F::cast_from(0.28582678745379824648e-3_f64) * t42391 * t6263 + F::cast_from(0.31758531939310916276e-3_f64) * t65650 - F::cast_from(0.57165357490759649296e-3_f64) * t3188 * t19682 - F::cast_from(0.21437009059034868486e-3_f64) * t65654 * t3164 - F::cast_from(0.28582678745379824648e-3_f64) * t11994 * t19702;
    t65659
}
