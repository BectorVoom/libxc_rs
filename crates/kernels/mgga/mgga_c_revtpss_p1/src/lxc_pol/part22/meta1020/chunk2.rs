//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3540/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3540<F: Float>(t11921: F, t19414: F, t247: F, t4837: F, t11710: F, t20078: F, t3091: F, t11922: F, t11927: F, t19621: F, t1045: F, t1062: F, t11774: F, t11866: F, t15691: F, t15728: F, t15809: F, t1592: F, t15950: F, t16089: F, t16095: F, t16154: F, t19705: F, t19838: F, t19878: F, t20083: F, t20101: F, t3075: F, t3092: F, t43038: F, t4578: F, t4839: F, t53885: F, t54695: F, t55033: F, t6273: F) -> F {
    let t67237 = t4837 * t247 * t11921 * t19414;
    let t67249 = t3091 * t11710 * t20078;
    let t67253 = t11927 * t11922 * t19621;
    let t67257 = -F::cast_from(0.42874018118069736972e-3_f64) * t43038 * t6273 - F::cast_from(0.85748036236139473944e-3_f64) * t11866 * t19838 + F::cast_from(0.11433071498151929859e-2_f64) * t16095 * t3092 * t4578 * t15950 + F::cast_from(0.17149607247227894789e-2_f64) * t54695 * t1062 * t4839 + F::cast_from(0.17149607247227894789e-2_f64) * t19878 * t16154 - F::cast_from(0.45732285992607719436e-2_f64) * t15728 * t20083 + F::cast_from(0.57165357490759649296e-3_f64) * t67237 + F::cast_from(0.57165357490759649296e-3_f64) * t16089 * t3092 * t19705 * t15809 - F::cast_from(0.28582678745379824648e-3_f64) * t11774 * t15691 * t1045 * t1592 * t3075 + F::cast_from(0.19055119163586549765e-3_f64) * t67249 + F::new(2.0) / F::new(243.0) * t55033 + F::cast_from(0.11433071498151929859e-2_f64) * t67253 + F::cast_from(0.5081365110289746604e-2_f64) * t53885 * t20101;
    t67257
}
