//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3247/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3247<F: Float>(t14718: F, t18637: F, t2661: F, t2662: F, t50583: F, t6035: F, t18408: F, t837: F, t50377: F, t50381: F, t50383: F, t50385: F, t50387: F, t50389: F, t50394: F, t50399: F) -> F {
    let t61612 = t2661 * t2662 * t14718 * t18637;
    let t61616 = t2661 * t2662 * t50583 * t6035;
    let t61620 = t2661 * t2662 * t18408 * t837;
    let t61622 = -F::cast_from(0.16065646176094875956e-5_f64) * t50377 + F::cast_from(0.22589491248727328396e-6_f64) * t50381 - F::cast_from(0.90702367218671976884e-1_f64) * t50383 - F::cast_from(0.10276933901433255264e-1_f64) * t50385 + F::cast_from(0.60976381323476959249e-2_f64) * t50387 + F::cast_from(0.2168320119862840671e-2_f64) * t50389 - F::cast_from(0.57165357490759649296e-3_f64) * t50394 + F::cast_from(0.17149607247227894789e-2_f64) * t50399 - F::cast_from(0.11433071498151929859e-3_f64) * t61612 - F::cast_from(0.11433071498151929859e-3_f64) * t61616 + F::cast_from(0.14291339372689912324e-4_f64) * t61620;
    t61622
}
