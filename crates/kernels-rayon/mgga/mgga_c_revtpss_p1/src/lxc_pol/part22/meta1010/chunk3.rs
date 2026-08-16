//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3466/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3466(t3140: f64, t6235: f64, t3149: f64, t19696: f64, t3168: f64, t15830: f64, t4817: f64, t1042: f64, t1047: f64, t15707: f64, t15811: f64, t15952: f64, t16210: f64, t1675: f64, t19649: f64, t19697: f64, t247: f64, t2853: f64, t2862: f64, t3116: f64, t3136: f64, t3157: f64, t3181: f64, t42939: f64, t4834: f64, t4837: f64, t4875: f64, t53692: f64, t54838: f64, t6244: f64, t6308: f64, t64835: f64) -> (f64, f64) {
    let t65338 = t6235 * t3140;
    let t65339 = t65338 * t3149;
    let t65342 = t19696 * t3168;
    let t65347 = t15830 * t4817;
    let t65353 = 0.1270341277572436651e-2_f64 * t4834 * t16210 + 0.28582678745379824648e-3_f64 * t4837 * t1042 * t19649 * t2862 + 0.47637797908966374413e-3_f64 * t4837 * t1042 * t3181 * t6244 * t2853 - 0.57165357490759649296e-3_f64 * t53692 * t4875 - 0.57165357490759649296e-3_f64 * t15707 * t15952 - 0.28582678745379824648e-3_f64 * t15707 * t15811 + 0.14481890564325777821e-1_f64 * t42939 * t6308 + 0.21437009059034868486e-3_f64 * t19697 * t3136 + 0.42874018118069736972e-3_f64 * t65339 * t3157 - 0.22866142996303859718e-2_f64 * t65342 * t1047 + 0.96545937095505185476e-2_f64 * t54838 * t1675 - 0.20325460441158986416e-2_f64 * t65347 + 0.42874018118069736972e-3_f64 * t4837 * t247 * t3116 * t64835;
    (t65338, t65353)
}
