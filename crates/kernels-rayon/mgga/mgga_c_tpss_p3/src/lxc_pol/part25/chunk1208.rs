//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1208/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1208(t19818: f64, t20047: f64, t1006: f64, t1398: f64, t33: f64, t3724: f64, t1497: f64, t750: f64, t821: f64, t4478: f64, t7383: f64, t18710: f64, t6245: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20048 = t20047 * t19818;
    let t20050 = t1006 * t1398;
    let t20054 = t33 * t3724;
    let t20058 = t1497 * t750;
    let t20065 = t1497 * t821;
    let t20134 = t7383 * t4478;
    let t20137 = t18710 * t6245;
    (t20048, t20050, t20054, t20058, t20065, t20134, t20137)
}
