//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3540/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3540(t11921: f64, t19414: f64, t247: f64, t4837: f64, t11710: f64, t20078: f64, t3091: f64, t11922: f64, t11927: f64, t19621: f64, t1045: f64, t1062: f64, t11774: f64, t11866: f64, t15691: f64, t15728: f64, t15809: f64, t1592: f64, t15950: f64, t16089: f64, t16095: f64, t16154: f64, t19705: f64, t19838: f64, t19878: f64, t20083: f64, t20101: f64, t3075: f64, t3092: f64, t43038: f64, t4578: f64, t4839: f64, t53885: f64, t54695: f64, t55033: f64, t6273: f64) -> f64 {
    let t67237 = t4837 * t247 * t11921 * t19414;
    let t67249 = t3091 * t11710 * t20078;
    let t67253 = t11927 * t11922 * t19621;
    let t67257 = -0.42874018118069736972e-3_f64 * t43038 * t6273 - 0.85748036236139473944e-3_f64 * t11866 * t19838 + 0.11433071498151929859e-2_f64 * t16095 * t3092 * t4578 * t15950 + 0.17149607247227894789e-2_f64 * t54695 * t1062 * t4839 + 0.17149607247227894789e-2_f64 * t19878 * t16154 - 0.45732285992607719436e-2_f64 * t15728 * t20083 + 0.57165357490759649296e-3_f64 * t67237 + 0.57165357490759649296e-3_f64 * t16089 * t3092 * t19705 * t15809 - 0.28582678745379824648e-3_f64 * t11774 * t15691 * t1045 * t1592 * t3075 + 0.19055119163586549765e-3_f64 * t67249 + 2.0_f64 / 243.0_f64 * t55033 + 0.11433071498151929859e-2_f64 * t67253 + 0.5081365110289746604e-2_f64 * t53885 * t20101;
    t67257
}
