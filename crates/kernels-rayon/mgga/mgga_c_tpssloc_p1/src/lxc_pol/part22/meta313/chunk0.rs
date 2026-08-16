//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1490/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1490(t15338: f64, t3451: f64, t3447: f64, t14818: f64, t14781: f64, t14710: f64, t1716: f64, t698: f64, t1174: f64, t3435: f64, t4889: f64, t135: f64, t4930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15339 = t15338 * t3451;
    let t15341 = 0.18518518518518518518e-3_f64 * t3447 * t15339;
    let t15347 = 2.0_f64 / 27.0_f64 * t14818;
    let t15348 = 4.0_f64 / 9.0_f64 * t14781;
    let t15349 = 2.0_f64 / 9.0_f64 * t14710;
    let t15363 = t698 * t1716;
    let t15364 = t1174 * t15363;
    let t15366 = t4889 * t3435;
    let t15372 = t135 * t4930;
    (t15339, t15341, t15347, t15348, t15349, t15363, t15364, t15366, t15372)
}
