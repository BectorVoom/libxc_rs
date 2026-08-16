//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2995/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2995<F: Float>(t15957: F, t357: F, t11710: F, t23907: F, t3091: F, t23912: F, t1668: F, t905: F, t11672: F, t11675: F, t11703: F, t11927: F, t16226: F, t19611: F, t19636: F, t19726: F, t19738: F, t19776: F, t23908: F, t23913: F, t23917: F, t23921: F, t23964: F, t3092: F, t3117: F, t3155: F, t4786: F, t4788: F, t53800: F, t54037: F, t606: F, t6092: F, t6096: F, t66261: F, t66288: F, t66304: F, t66777: F, t67528: F) -> (F, F) {
    let t79410 = t15957 * t357;
    let t79428 = t3091 * t11710 * t23907;
    let t79439 = t3091 * t11710 * t23912;
    let t79450 = t1668 * t905;
    let t79456 = F::cast_from(0.7145669686344956162e-3_f64) * t11675 * t23917 + F::cast_from(0.7145669686344956162e-3_f64) * t3091 * t11703 * t6092 * t79410 - F::cast_from(0.85748036236139473944e-3_f64) * t11675 * t23921 - F::cast_from(0.85748036236139473944e-3_f64) * t3091 * t3092 * t6096 * t79410 + F::cast_from(0.85748036236139473947e-3_f64) * t19738 * t19726 - F::cast_from(0.25724410870841842184e-2_f64) * t53800 * t19636 - F::cast_from(0.22866142996303859718e-2_f64) * t11672 * t23908 + F::cast_from(0.28582678745379824648e-3_f64) * t79428 + F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t23964 * t4786 + F::cast_from(0.42874018118069736972e-3_f64) * t67528 * t4788 - F::cast_from(0.22866142996303859718e-2_f64) * t11672 * t23913 + F::cast_from(0.28582678745379824648e-3_f64) * t79439 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t3092 * t19611 * t19776 + F::cast_from(0.42874018118069736972e-3_f64) * t11675 * t23913 + F::cast_from(0.57165357490759649295e-3_f64) * t66261 - t54037 - F::cast_from(0.25724410870841842183e-2_f64) * t66288 - F::cast_from(0.85748036236139473944e-3_f64) * t66304 + F::cast_from(0.85748036236139473947e-3_f64) * t16226 * t66777 * t3155 * t79450 * t606;
    (t79410, t79456)
}
