//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 973/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk973(t2593: f64, t3295: f64, t2599: f64, t3308: f64, t1577: f64, t10710: f64, t7257: f64, t10728: f64, t7261: f64, t10708: f64, t2124: f64, t8070: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11691 = t3295 * t2593;
    let t11693 = t3308 * t2599;
    let t11694 = t1577 * t11693;
    let t11696 = t10710 * t7257;
    let t11697 = t10728 * t11696;
    let t11699 = t10710 * t7261;
    let t11700 = t10708 * t11699;
    let t11702 = t2124 * t8070;
    (t11691, t11693, t11694, t11696, t11697, t11699, t11700, t11702)
}
