//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 926/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk926(t275: f64, t277: f64, t8662: f64, t267: f64, t270: f64, t279: f64, t8660: f64, t2529: f64, t844: f64, t269: f64, t2532: f64, t284: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8664 = t275 * t8662 * t277;
    let t8665 = 0.36514074074074074075e0_f64 * t8664;
    let t8678 = 1.0_f64/pow_3_2(t267);
    let t8684 = 1.0_f64 / t270 / t279 / 4.0_f64;
    let t8687 = 28.0_f64 / 27.0_f64 * t8660;
    let t8709 = 1.0_f64 / t2529 / t844;
    let t8710 = t269 * t8709;
    let t8712 = 1.0_f64 / t2532 / t284;
    (t8664, t8665, t8678, t8684, t8687, t8710, t8712)
}
