//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1005/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1005(t1174: f64, t15285: f64, t11583: f64, t3961: f64, t11529: f64, t1709: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64, t3448: f64, t4928: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15287 = 0.18518518518518518518e-3_f64 * t1174 * t15285;
    let t15293 = t11583 * t3961;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15307 = t4889 * t3432;
    let t15313 = t3450 * t3966;
    let t15320 = t3448 * t4928;
    (t15287, t15293, t15300, t15307, t15313, t15320)
}
