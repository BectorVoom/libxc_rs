//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1493/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1493<F: Float>(t1065: F, t3133: F, t372: F, t1043: F, t1045: F, t11165: F, t3181: F, t11156: F, t1011: F, t1028: F, t11637: F, t11774: F, t15700: F, t15701: F, t16012: F, t16226: F, t16229: F, t41248: F, t41263: F, t42279: F, t42282: F, t42284: F, t42288: F, t42290: F, t4786: F, t4919: F) -> (F, F, F, F) {
    let t42300 = t372 * t1065 * t3133;
    let t42309 = t372 * t1065 * t1043;
    let t42310 = t1045 * t11165;
    let t42315 = t372 * t3181 * t1043;
    let t42316 = t1045 * t11156;
    let t42320 = -F::cast_from(0.85748036236139473944e-3_f64) * t42279 * t1028 + F::cast_from(0.18292914397043087775e-1_f64) * t42282 - F::cast_from(0.17149607247227894789e-2_f64) * t42284 - F::cast_from(0.57165357490759649296e-3_f64) * t42288 + F::cast_from(0.13719685797782315831e-1_f64) * t42290 * t1028 + F::cast_from(7.0_f64) / F::cast_from(108.0_f64) * t1011 * t16012 * t41248 - t1011 * t4919 * t41263 / F::cast_from(6.0_f64) + F::cast_from(0.34299214494455789578e-2_f64) * t16226 * t42300 * t16229 + F::cast_from(0.34299214494455789577e-2_f64) * t11774 * t15701 * t11637 * t4786 - F::cast_from(0.34299214494455789578e-2_f64) * t15700 * t42309 * t42310 + F::cast_from(0.28582678745379824648e-2_f64) * t15700 * t42315 * t42316;
    (t42300, t42309, t42315, t42320)
}
