//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 754/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk754(t11679: f64, t161: f64, t3601: f64, t830: f64, t723: f64, t21502: f64, t3614: f64, t2610: f64, t701: f64, t7290: f64, t11576: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35435 = t11679 * t161;
    let t35439 = t830 * t3601;
    let t35440 = t35439 * t161;
    let t35445 = t3601 * t723;
    let t35446 = t21502 * t35445;
    let t35450 = t3614 * t723;
    let t35451 = t2610 * t35450;
    let t35499 = t3614 * t701;
    let t35500 = t2610 * t35499;
    let t35549 = t3601 * t701;
    let t35550 = t7290 * t35549;
    let t35558 = t325 * t11576;
    (t35435, t35439, t35440, t35445, t35446, t35450, t35451, t35500, t35550, t35558)
}
