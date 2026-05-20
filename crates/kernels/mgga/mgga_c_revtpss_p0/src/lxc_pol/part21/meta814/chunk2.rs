//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2984/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2984<F: Float>(t1045: F, t606: F, t3118: F, t1053: F, t15670: F, t11937: F, t15671: F, t11262: F, t3127: F, t4824: F, t11672: F, t11774: F, t11994: F, t1469: F, t15606: F, t15959: F, t16144: F, t16201: F, t3188: F, t3208: F, t372: F, t42425: F, t42675: F, t42795: F, t42798: F, t4806: F, t4825: F) -> (F, F) {
    let t54397 = t1045 * t606;
    let t54398 = t3118 * t54397;
    let t54404 = t15670 * t1053;
    let t54407 = t15671 * t11937;
    let t54414 = t3127 * t11262 * t4824;
    let t54418 = -F::cast_from(0.68598428988911579154e-2_f64) * t42675 * t15606 + F::cast_from(0.28582678745379824648e-3_f64) * t42795 + F::cast_from(0.85748036236139473944e-3_f64) * t42798 - F::cast_from(0.14291339372689912324e-2_f64) * t11774 * t372 * t4806 * t1469 * t54398 - F::cast_from(0.14481890564325777821e-1_f64) * t42425 * t4825 - F::cast_from(0.68598428988911579154e-2_f64) * t54404 * t3208 + F::cast_from(0.85748036236139473944e-3_f64) * t54407 - F::cast_from(0.45732285992607719436e-2_f64) * t11672 * t15959 - F::cast_from(0.42874018118069736973e-2_f64) * t3188 * t16201 + F::cast_from(0.95275595817932748825e-4_f64) * t54414 + F::cast_from(0.85748036236139473944e-3_f64) * t11994 * t16144;
    (t54398, t54418)
}
