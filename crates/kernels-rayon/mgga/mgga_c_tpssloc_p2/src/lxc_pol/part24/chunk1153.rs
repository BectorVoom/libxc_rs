//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1153/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1153(t23066: f64, t6591: f64, t229: f64, t6546: f64, t805: f64, t2628: f64, t2633: f64, t6605: f64, t243: f64, t598: f64, t213: f64, t1894: f64, t236: f64, t2379: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23067 = t6591 * t23066;
    let t23069 = t6546 * t229;
    let t23070 = t23069 * t805;
    let t23071 = 7.0_f64 / 72.0_f64 * t23070;
    let t23072 = t2628 * t2633;
    let t23073 = t6605 * t23072;
    let t23075 = t243 * t243;
    let t23076 = 1.0_f64 / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23080 = t1894 * t236 * t2379;
    (t23067, t23069, t23071, t23072, t23073, t23075, t23076, t23077, t23078, t23080)
}
