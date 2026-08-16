//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1239/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1239(t19476: f64, t4419: f64, t1642: f64, t18450: f64, t18454: f64, t4425: f64, t4462: f64, t5721: f64, t4466: f64, t4473: f64, t1646: f64, t18464: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19477 = t19476 * t4419;
    let t19479 = t18450 * t1642;
    let t19481 = t18454 * t4425;
    let t19483 = t5721 * t4462;
    let t19485 = t18454 * t4466;
    let t19489 = t18454 * t4473;
    let t19491 = t18464 * t1646;
    (t19477, t19479, t19481, t19483, t19485, t19489, t19491)
}
