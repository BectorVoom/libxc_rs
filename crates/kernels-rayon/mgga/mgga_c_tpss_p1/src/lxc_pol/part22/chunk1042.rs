//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1042/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1042(t3819: f64, t876: f64, t1429: f64, t2574: f64, t10982: f64, t10989: f64, t11049: f64, t10992: f64, t10994: f64, t11041: f64, t11044: f64, t11047: f64, t11051: f64, t8647: f64, t8871: f64, t8872: f64) -> (f64, f64, f64, f64) {
    let t11289 = t3819 * t876;
    let t11294 = t1429 * t2574;
    let t11309 = 0.34431666666666666666e0_f64 * t10982;
    let t11312 = 0.13892666666666666667e0_f64 * t10989;
    let t11319 = 0.27785333333333333334e0_f64 * t11049;
    let t11321 = t11312 - 0.104195e0_f64 * t10992 - 0.11577222222222222222e0_f64 * t10994 - 0.13892666666666666667e0_f64 * t8647 - t8871 - t8872 + 0.3529725e1_f64 * t11041 - 0.62517e0_f64 * t11044 + 0.20839e0_f64 * t11047 - t11319 + 0.46308888888888888889e-1_f64 * t11051;
    (t11289, t11294, t11309, t11321)
}
