//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1046/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1046(t1441: f64, t2593: f64, t1429: f64, t2549: f64, t11207: f64, t904: f64, t1437: f64, t2551: f64, t10965: f64, t10968: f64, t10970: f64, t10972: f64, t11103: f64, t11121: f64, t11146: f64, t11218: f64, t2552: f64, t2575: f64, t2589: f64, t2596: f64, t3883: f64, t896: f64) -> f64 {
    let t11362 = t1441 * t2593;
    let t11366 = t1429 * t2549;
    let t11371 = t11207 * t904;
    let t11374 = t1437 * t2551;
    let t11377 = -t10965 - t10968 - t10970 - t10972 - t11103 - 0.11696447245269292414e1_f64 * t11362 * t2596 + t11146 - 0.19751673498613801407e-1_f64 * t11121 + t11218 - 2.0_f64 * t11366 * t2552 + 0.11696447245269292414e1_f64 * t2589 * t3883 + 0.5848223622634646207e0_f64 * t896 * t11371 + 6.0_f64 * t2575 * t11374;
    t11377
}
