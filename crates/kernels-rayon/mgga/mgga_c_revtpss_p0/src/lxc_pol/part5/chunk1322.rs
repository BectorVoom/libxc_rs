//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1322/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1322(t20977: f64, t3720: f64, t19666: f64, t5268: f64, t1042: f64, t17202: f64, t19661: f64, t1261: f64, t12855: f64, t12967: f64, t17362: f64, t17569: f64, t17709: f64, t17747: f64, t20959: f64, t20963: f64, t20966: f64, t20974: f64, t3647: f64, t5299: f64, t5391: f64, t5397: f64, t6611: f64, t6679: f64) -> f64 {
    let t20978 = t3720 * t20977;
    let t20981 = t5268 * t19666;
    let t20982 = t1042 * t20981;
    let t20985 = t17202 * t19661;
    let t20986 = t1042 * t20985;
    let t20993 = 0.12862205435420921092e-2_f64 * t17709 * t20959 - 0.12862205435420921092e-2_f64 * t17747 * t20963 + 11.0_f64 / 324.0_f64 * t20966 + 0.15244095330869239812e-2_f64 * t5391 * t5397 - 0.14291339372689912324e-3_f64 * t3647 * t6679 - 0.95275595817932748827e-4_f64 * t20974 - 0.95275595817932748827e-4_f64 * t17362 - 0.42874018118069736972e-3_f64 * t12855 * t20978 - 0.57165357490759649296e-3_f64 * t1261 * t20982 - 0.85748036236139473944e-3_f64 * t1261 * t20986 + 0.28582678745379824648e-3_f64 * t17569 * t5299 + 0.42874018118069736972e-3_f64 * t12967 * t6611;
    t20993
}
