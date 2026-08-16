//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 605/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk605(t2360: f64, t2670: f64, t564: f64, t2063: f64, t2527: f64, t5185: f64, t5184: f64, t5182: f64, t2441: f64, t5193: f64, t5192: f64, t682: f64, t7715: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8476 = t2360 * t2670;
    let t8477 = t564 * t8476;
    let t8479 = t2063 * t2527;
    let t8480 = t5185 * t8479;
    let t8481 = t5184 * t8480;
    let t8482 = t5182 * t8481;
    let t8484 = t2063 * t2441;
    let t8485 = t5193 * t8484;
    let t8486 = t5192 * t8485;
    let t8487 = t5182 * t8486;
    let t8491 = t682 * t7715;
    (t8476, t8477, t8480, t8481, t8482, t8485, t8486, t8487, t8491)
}
