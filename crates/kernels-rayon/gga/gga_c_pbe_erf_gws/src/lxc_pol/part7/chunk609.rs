//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 609/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk609(t4687: f64, t1215: f64, t155: f64, t1319: f64, t331: f64, t449: f64, t388: f64, t174: f64, t405: f64, t1268: f64, t1286: f64, t1290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4688 = 0.10685e0_f64 * t4687;
    let t4689 = t155 * t1215;
    let t4697 = t155 * t1319;
    let t4701 = t331 * t449;
    let t4708 = t331 * t388;
    let t4710 = t174 * t4708 * t405;
    let t4711 = 0.71233333333333333334e-1_f64 * t4710;
    let t4713 = t174 * t1268 * t1286;
    let t4714 = 0.53425e-1_f64 * t4713;
    let t4715 = t155 * t1290;
    (t4688, t4689, t4697, t4701, t4708, t4710, t4711, t4713, t4714, t4715)
}
