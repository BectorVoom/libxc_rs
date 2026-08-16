//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 812/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk812(t1011: f64, t19045: f64, t1212: f64, t1226: f64, t6169: f64, t486: f64, t6218: f64, t5001: f64, t5018: f64, t1730: f64, t5023: f64, t1193: f64, t6109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19046 = t19045 * t1011;
    let t19047 = t19046 * t1212;
    let t19051 = t6169 * t1226;
    let t19056 = t486 * t6218;
    let t19080 = t5001 * t5018;
    let t19083 = t1730 * t5023;
    let t19090 = t6109 * t1193;
    (t19047, t19051, t19056, t19080, t19083, t19090)
}
