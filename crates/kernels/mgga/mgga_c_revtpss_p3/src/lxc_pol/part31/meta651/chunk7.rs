//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2159/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2159<F: Float>(t100135: F, t100168: F, t100255: F, t1047: F, t106877: F, t106913: F, t106929: F, t106943: F, t106968: F, t106990: F, t107012: F, t107035: F, t107048: F, t107082: F, t107103: F, t107120: F, t107144: F, t107159: F, t107183: F, t107197: F, t1671: F, t19702: F, t19800: F, t20096: F, t20101: F, t25512: F, t25522: F, t25526: F, t25569: F, t27450: F, t27489: F, t4803: F, t4808: F, t4825: F, t4869: F, t6263: F, t6302: F, t6308: F, t6312: F, t6323: F, t7122: F, t93567: F, t93696: F, t93764: F, t93783: F, t93796: F, t99983: F) -> F {
    let t107201 = -t93696 / F::new(1296.0) + t107144 + t106968 + t107103 + t107159 + t106929 + t99983 + t107082 + t106990 + t106943 + F::cast_from(0.28582678745379824648e-3_f64) * t106877 + t107120 + t106913 + F::cast_from(0.11433071498151929859e-2_f64) * t100135 * t20096 - F::cast_from(0.95275595817932748827e-3_f64) * t100135 * t20101 - F::cast_from(0.45732285992607719437e-2_f64) * t93567 * t6308 + F::cast_from(0.42874018118069736972e-3_f64) * t25512 * t6302 + F::cast_from(0.42874018118069736972e-3_f64) * t7122 * t19800 + F::cast_from(0.85748036236139473944e-3_f64) * t93783 * t6308 - F::cast_from(0.42874018118069736972e-3_f64) * t93796 * t6312 - F::cast_from(0.28582678745379824648e-3_f64) * t25522 * t19702 - F::cast_from(0.22866142996303859718e-2_f64) * t25526 * t6302 + F::cast_from(0.28582678745379824648e-3_f64) * t25569 * t6323 - F::cast_from(0.11433071498151929859e-2_f64) * t27489 * t4803 + F::cast_from(0.95275595817932748827e-3_f64) * t27489 * t4808 + F::cast_from(0.42874018118069736972e-3_f64) * t107048 * t1047 - F::cast_from(0.57165357490759649296e-3_f64) * t100255 * t4825 + F::cast_from(0.85748036236139473944e-3_f64) * t100168 * t1671 + F::cast_from(0.85748036236139473944e-3_f64) * t27450 * t4869 - F::cast_from(0.57165357490759649296e-3_f64) * t93764 * t6263 + t107012 + t107197 + t107035 + t107183;
    t107201
}
