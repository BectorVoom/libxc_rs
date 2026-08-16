//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 566/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk566(t2328: f64, t900: f64, t2295: f64, t2297: f64, t890: f64, t898: f64, t2312: f64, t881: f64, t2317: f64, t2320: f64, t154: f64, t386: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2330 = 0.11696447245269292414e1_f64 * t2328 * t900;
    let t2332 = t2295 * t2297 * t890;
    let t2334 = 0.11696447245269292414e1_f64 * t898 * t2332;
    let t2336 = t881 * t2312 * t890;
    let t2338 = 0.5848223622634646207e0_f64 * t898 * t2336;
    let t2339 = t2317 * t2297;
    let t2340 = t2339 * t2320;
    let t2342 = 0.17315859105681463759e2_f64 * t898 * t2340;
    let t2344 = t154 * t486 * t386;
    (t2330, t2332, t2334, t2336, t2338, t2340, t2342, t2344)
}
