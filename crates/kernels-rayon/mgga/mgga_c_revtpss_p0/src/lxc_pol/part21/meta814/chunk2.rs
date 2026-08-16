//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2984/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2984(t1045: f64, t606: f64, t3118: f64, t1053: f64, t15670: f64, t11937: f64, t15671: f64, t11262: f64, t3127: f64, t4824: f64, t11672: f64, t11774: f64, t11994: f64, t1469: f64, t15606: f64, t15959: f64, t16144: f64, t16201: f64, t3188: f64, t3208: f64, t372: f64, t42425: f64, t42675: f64, t42795: f64, t42798: f64, t4806: f64, t4825: f64) -> (f64, f64) {
    let t54397 = t1045 * t606;
    let t54398 = t3118 * t54397;
    let t54404 = t15670 * t1053;
    let t54407 = t15671 * t11937;
    let t54414 = t3127 * t11262 * t4824;
    let t54418 = -0.68598428988911579154e-2_f64 * t42675 * t15606 + 0.28582678745379824648e-3_f64 * t42795 + 0.85748036236139473944e-3_f64 * t42798 - 0.14291339372689912324e-2_f64 * t11774 * t372 * t4806 * t1469 * t54398 - 0.14481890564325777821e-1_f64 * t42425 * t4825 - 0.68598428988911579154e-2_f64 * t54404 * t3208 + 0.85748036236139473944e-3_f64 * t54407 - 0.45732285992607719436e-2_f64 * t11672 * t15959 - 0.42874018118069736973e-2_f64 * t3188 * t16201 + 0.95275595817932748825e-4_f64 * t54414 + 0.85748036236139473944e-3_f64 * t11994 * t16144;
    (t54398, t54418)
}
