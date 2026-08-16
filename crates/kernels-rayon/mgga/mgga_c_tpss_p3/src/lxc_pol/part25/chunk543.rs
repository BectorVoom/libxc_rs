//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 543/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk543(t294: f64, t891: f64, t2464: f64, t928: f64, t359: f64, t361: f64, t651: f64, t355: f64, t958: f64, t962: f64, t917: f64, t921: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2629 = t294 * t891;
    let t2644 = t928 * t2464;
    let t2650 = t359 * t651 * t361;
    let t2652 = t355 * t2650 / 13824.0_f64;
    let t2660 = t958 * t962;
    let t2665 = t917 * t921;
    (t2629, t2644, t2650, t2652, t2660, t2665)
}
