//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 359/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk359<F: Float>(t468: F, t732: F, t20: F, t614: F, t21: F, t6: F, t263: F, t124: F, t386: F, t385: F, t7: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1388 = t732 * t468;
    let t1389 = F::cast_from(0.11696447245269292414e1_f64) * t1388;
    let t1390 = t614 * t20;
    let t1391 = t21 * t6;
    let t1392 = t1391 * t263;
    let t1393 = t1390 * t1392;
    let t1395 = t386 * t124;
    let t1396 = t385 * t1395;
    let t1398 = t7 * t124;
    (t1388, t1389, t1390, t1391, t1392, t1393, t1395, t1396, t1398)
}
