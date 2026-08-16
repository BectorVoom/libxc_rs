//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 346/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk346(t468: f64, t732: f64, t20: f64, t614: f64, t21: f64, t6: f64, t263: f64, t124: f64, t386: f64, t385: f64, t7: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1388 = t732 * t468;
    let t1390 = t614 * t20;
    let t1391 = t21 * t6;
    let t1392 = t1391 * t263;
    let t1393 = t1390 * t1392;
    let t1395 = t386 * t124;
    let t1396 = t385 * t1395;
    let t1398 = t7 * t124;
    (t1388, t1390, t1391, t1392, t1393, t1395, t1396, t1398)
}
