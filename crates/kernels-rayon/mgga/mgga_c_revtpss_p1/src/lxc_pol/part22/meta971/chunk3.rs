//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3247/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3247(t14718: f64, t18637: f64, t2661: f64, t2662: f64, t50583: f64, t6035: f64, t18408: f64, t837: f64, t50377: f64, t50381: f64, t50383: f64, t50385: f64, t50387: f64, t50389: f64, t50394: f64, t50399: f64) -> f64 {
    let t61612 = t2661 * t2662 * t14718 * t18637;
    let t61616 = t2661 * t2662 * t50583 * t6035;
    let t61620 = t2661 * t2662 * t18408 * t837;
    let t61622 = -0.16065646176094875956e-5_f64 * t50377 + 0.22589491248727328396e-6_f64 * t50381 - 0.90702367218671976884e-1_f64 * t50383 - 0.10276933901433255264e-1_f64 * t50385 + 0.60976381323476959249e-2_f64 * t50387 + 0.2168320119862840671e-2_f64 * t50389 - 0.57165357490759649296e-3_f64 * t50394 + 0.17149607247227894789e-2_f64 * t50399 - 0.11433071498151929859e-3_f64 * t61612 - 0.11433071498151929859e-3_f64 * t61616 + 0.14291339372689912324e-4_f64 * t61620;
    t61622
}
