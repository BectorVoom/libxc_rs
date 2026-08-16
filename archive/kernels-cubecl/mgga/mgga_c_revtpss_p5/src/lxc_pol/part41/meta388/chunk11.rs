//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1305/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1305<F: Float>(t20090: F, t3117: F, t1651: F, t2857: F, t4181: F, t3092: F, t2852: F, t11703: F, t19611: F, t4910: F, t11859: F, t15850: F, t16095: F, t16165: F, t16218: F, t16220: F, t1675: F, t20075: F, t20079: F, t20083: F, t3091: F, t3115: F, t4837: F) -> F {
    let t20091 = t3117 * t20090;
    let t20094 = t1651 * t2857;
    let t20095 = t20094 * t4181;
    let t20096 = t3092 * t20095;
    let t20099 = t1651 * t2852;
    let t20100 = t20099 * t4181;
    let t20101 = t11703 * t20100;
    let t20104 = t19611 * t4910;
    let t20105 = t3117 * t20104;
    let t20108 = -F::cast_from(0.42874018118069736972e-3_f64) * t11859 * t20075 + t16165 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t20079 + F::cast_from(0.42874018118069736972e-3_f64) * t4837 * t20083 + F::cast_from(0.28582678745379824648e-3_f64) * t15850 * t1675 + t16218 - t16220 / F::cast_from(648.0_f64) - F::cast_from(0.42874018118069736972e-3_f64) * t3115 * t20091 + F::cast_from(0.57165357490759649296e-3_f64) * t16095 * t20096 - F::cast_from(0.47637797908966374413e-3_f64) * t16095 * t20101 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t20105;
    t20108
}
