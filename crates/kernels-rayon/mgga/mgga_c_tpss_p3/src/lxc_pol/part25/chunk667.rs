//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 667/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk667(t1080: f64, t3001: f64, t4205: f64, t1089: f64, t1095: f64, t1554: f64, t1300: f64, t924: f64) -> (f64, f64, f64, f64, f64) {
    let t4206 = t3001 * t1080;
    let t4207 = t4205 * t4206;
    let t4209 = 0.17315859105681463759e2_f64 * t1089 * t4207;
    let t4210 = t1554 * t1095;
    let t4212 = t1300 * t924;
    (t4206, t4207, t4209, t4210, t4212)
}
