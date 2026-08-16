//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 300/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk300(t339: f64, t349: f64, t956: f64, t127: f64, t359: f64, t361: f64, t355: f64, t353: f64, t357: f64) -> (f64, f64, f64, f64) {
    let t958 = t339 * t349 * t956;
    let t962 = t359 * t127 * t361;
    let t964 = t355 * t962 / 4608.0_f64;
    let t965 = t353 * t357;
    let t967 = t339 * t349 * t965;
    (t958, t962, t964, t967)
}
