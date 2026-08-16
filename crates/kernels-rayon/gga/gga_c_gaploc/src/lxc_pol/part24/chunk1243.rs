//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1243/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1243(t10669: f64, t10674: f64, t169: f64, t1908: f64, t1935: f64, t1939: f64, t270: f64, t29471: f64, t29473: f64, t29476: f64, t299: f64, t32313: f64, t32585: f64, t32588: f64, t32591: f64, t32594: f64, t32597: f64, t3434: f64, t3452: f64, t650: f64, t706: f64) -> f64 {
    let t32598 = -0.20508069947045931424e-1_f64 * t650 * t10669 - 0.76905262301422242837e-2_f64 * t1935 * t3452 + 0.76905262301422242837e-2_f64 * t270 * t706 * t32313 * t169 * t299 - 0.34180116578409885707e-2_f64 * t1908 * t3452 + 0.20508069947045931424e-1_f64 * t650 * t10674 + 0.20508069947045931424e-1_f64 * t1939 * t3434 - t29471 + t29473 - t32585 + t32588 + t32591 - t32594 - t29476 + t32597;
    t32598
}
