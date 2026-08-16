//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1459/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1459(t1853: f64, t3721: f64, t12281: f64, t123: f64, t12312: f64, t12318: f64, t169: f64, t1841: f64, t1897: f64, t1908: f64, t1939: f64, t270: f64, t29471: f64, t29473: f64, t299: f64, t32560: f64, t32585: f64, t32588: f64, t32591: f64, t32594: f64, t3727: f64, t39121: f64, t39181: f64, t5227: f64, t5524: f64, t650: f64, t706: f64, t734: f64, t779: f64) -> f64 {
    let t39454 = t3721 * t1853;
    let t39464 = t32560 - 0.8545029144602471425e-3_f64 * t5524 * t12318 - 0.20508069947045931424e-1_f64 * t650 * t12281 - 0.34180116578409885707e-2_f64 * t1908 * t3727 - 0.20508069947045931424e-1_f64 * t1939 * t3727 + 0.76905262301422242837e-2_f64 * t270 * t706 * t39181 * t169 * t299 - 0.15381052460284448567e-1_f64 * t1897 * t779 * t39454 - t29471 + t29473 - 0.17090058289204942853e-2_f64 * t5227 * t12312 - 0.17090058289204942853e-2_f64 * t1841 * t39121 * t123 * t734 - t32585 + t32588 + t32591 - t32594;
    t39464
}
