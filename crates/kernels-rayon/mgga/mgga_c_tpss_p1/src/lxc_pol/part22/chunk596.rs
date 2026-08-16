//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 596/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk596(t1985: f64, t2644: f64, t926: f64, t359: f64, t361: f64, t651: f64, t355: f64, t350: f64, t40: f64, t586: f64, t339: f64, t349: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2645 = t2644 * t1985;
    let t2646 = t926 * t2645;
    let t2650 = t359 * t651 * t361;
    let t2652 = t355 * t2650 / 13824.0_f64;
    let t2655 = 1.0_f64 / t40 / t350 / t586;
    let t2657 = t339 * t349 * t2655;
    (t2645, t2646, t2650, t2652, t2655, t2657)
}
