//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 687/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk687(t13327: f64, t2268: f64, t12831: f64, t11288: f64, t921: f64, t3366: f64, t8045: f64, t3553: f64, t6556: f64, t4349: f64, t2355: f64, t3599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13329 = 0.28455006635676149599e-1_f64 * t2268 * t13327;
    let t13330 = 0.142275033178380748e-1_f64 * t12831;
    let t13334 = t11288 * t921;
    let t13338 = 4.0_f64 * t8045 * t3366;
    let t13342 = 2.0_f64 * t6556 * t3553;
    let t13343 = t3553 * t921;
    let t13345 = 6.0_f64 * t4349 * t13343;
    let t13349 = t2355 * t3599;
    (t13329, t13330, t13334, t13338, t13342, t13343, t13345, t13349)
}
