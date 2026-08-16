//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1488/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1488(t15281: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t11583: f64, t3961: f64, t11529: f64, t1709: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15282 = t15281 * t4936;
    let t15284 = 0.55555555555555555554e-3_f64 * t1174 * t15282;
    let t15285 = t3431 * t4912;
    let t15287 = 0.18518518518518518518e-3_f64 * t1174 * t15285;
    let t15293 = t11583 * t3961;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15307 = t4889 * t3432;
    let t15313 = t3450 * t3966;
    (t15282, t15284, t15285, t15287, t15293, t15299, t15300, t15307, t15313)
}
