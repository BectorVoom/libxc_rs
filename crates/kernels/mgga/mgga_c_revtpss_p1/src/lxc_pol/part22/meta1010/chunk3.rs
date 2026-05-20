//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3466/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3466<F: Float>(t3140: F, t6235: F, t3149: F, t19696: F, t3168: F, t15830: F, t4817: F, t1042: F, t1047: F, t15707: F, t15811: F, t15952: F, t16210: F, t1675: F, t19649: F, t19697: F, t247: F, t2853: F, t2862: F, t3116: F, t3136: F, t3157: F, t3181: F, t42939: F, t4834: F, t4837: F, t4875: F, t53692: F, t54838: F, t6244: F, t6308: F, t64835: F) -> (F, F) {
    let t65338 = t6235 * t3140;
    let t65339 = t65338 * t3149;
    let t65342 = t19696 * t3168;
    let t65347 = t15830 * t4817;
    let t65353 = F::cast_from(0.1270341277572436651e-2_f64) * t4834 * t16210 + F::cast_from(0.28582678745379824648e-3_f64) * t4837 * t1042 * t19649 * t2862 + F::cast_from(0.47637797908966374413e-3_f64) * t4837 * t1042 * t3181 * t6244 * t2853 - F::cast_from(0.57165357490759649296e-3_f64) * t53692 * t4875 - F::cast_from(0.57165357490759649296e-3_f64) * t15707 * t15952 - F::cast_from(0.28582678745379824648e-3_f64) * t15707 * t15811 + F::cast_from(0.14481890564325777821e-1_f64) * t42939 * t6308 + F::cast_from(0.21437009059034868486e-3_f64) * t19697 * t3136 + F::cast_from(0.42874018118069736972e-3_f64) * t65339 * t3157 - F::cast_from(0.22866142996303859718e-2_f64) * t65342 * t1047 + F::cast_from(0.96545937095505185476e-2_f64) * t54838 * t1675 - F::cast_from(0.20325460441158986416e-2_f64) * t65347 + F::cast_from(0.42874018118069736972e-3_f64) * t4837 * t247 * t3116 * t64835;
    (t65338, t65353)
}
