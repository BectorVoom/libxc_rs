//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1288/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1288(t3610: f64, t821: f64, t3724: f64, t750: f64, t1364: f64, t2433: f64, t14179: f64, t782: f64, t3664: f64, t783: f64, t18495: f64, t5736: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44329 = t3610 * t821;
    let t44350 = t3724 * t821;
    let t44470 = t750 * t3724;
    let t44474 = t1364 * t2433;
    let t44584 = t14179 * t782;
    let t44610 = t783 * t3664;
    let t60649 = t5736 * t18495;
    (t44329, t44350, t44470, t44474, t44584, t44610, t60649)
}
