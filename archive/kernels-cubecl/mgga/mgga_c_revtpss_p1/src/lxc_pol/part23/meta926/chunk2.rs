//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3005/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3005<F: Float>(t1592: F, t4866: F, t1011: F, t15993: F, t23499: F, t11875: F, t11922: F, t24012: F, t11632: F, t11703: F, t11774: F, t15689: F, t15691: F, t15696: F, t16095: F, t16226: F, t18936: F, t19985: F, t19996: F, t20038: F, t20039: F, t20078: F, t23984: F, t24013: F, t3092: F, t3155: F, t3162: F, t3241: F, t42675: F, t43082: F, t4574: F, t4873: F, t53741: F, t54811: F, t55331: F, t6266: F, t66187: F, t66702: F, t66777: F, t66951: F, t66966: F, t66972: F, t66981: F, t67006: F, t67015: F, t78524: F, t79247: F) -> (F, F) {
    let t79770 = t1592 * t4866;
    let t79811 = t1011 * t15993 * t23499;
    let t79818 = t11875 * t11922 * t24012;
    let t79822 = F::cast_from(0.85748036236139473944e-3_f64) * t66951 + F::cast_from(0.17149607247227894789e-2_f64) * t16226 * t15691 * t3155 * t79770 - F::cast_from(0.7145669686344956162e-3_f64) * t16095 * t11703 * t18936 * t78524 - F::cast_from(0.25724410870841842183e-2_f64) * t55331 * t3092 * t66966 * t4873 - F::cast_from(0.85748036236139473944e-3_f64) * t43082 * t66187 * t3155 * t20038 - F::cast_from(0.42874018118069736972e-3_f64) * t15689 * t66777 * t19985 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15696 * t20078 - F::cast_from(0.57165357490759649296e-3_f64) * t66972 - F::cast_from(0.7145669686344956162e-3_f64) * t15689 * t79247 * t3162 * t4574 + F::cast_from(0.42874018118069736972e-3_f64) * t54811 * t66187 * t66702 * t6266 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t66777 * t20039 - F::cast_from(0.34299214494455789578e-2_f64) * t42675 * t24013 - t66981 / F::cast_from(36.0_f64) - t3241 * t23984 / F::cast_from(27.0_f64) + t79811 / F::cast_from(216.0_f64) + F::cast_from(0.25724410870841842183e-2_f64) * t53741 * t66187 * t11632 * t19996 + F::cast_from(0.42874018118069736972e-3_f64) * t79818 + F::cast_from(0.45732285992607719437e-2_f64) * t67006 + F::cast_from(0.28582678745379824648e-3_f64) * t67015;
    (t79770, t79822)
}
