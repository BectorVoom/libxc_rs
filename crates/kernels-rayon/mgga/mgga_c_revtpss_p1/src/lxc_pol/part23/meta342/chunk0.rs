//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1644/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1644(t2516: f64, t4398: f64, t2496: f64, t2619: f64, t4302: f64, t4186: f64, t750: f64, t706: f64, t4395: f64, t4537: f64, t892: f64, t123: f64, t1534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14334 = t4398 * t2516;
    let t14336 = t4398 * t2496;
    let t14339 = t4302 * t2619;
    let t14341 = t750 * t4186;
    let t14343 = 8.0_f64 * t706 * t14341;
    let t14345 = 2.0_f64 * t4395 * t750;
    let t14353 = t4537 * t892;
    let t14362 = t1534 * t123;
    (t14334, t14336, t14339, t14341, t14343, t14345, t14353, t14362)
}
