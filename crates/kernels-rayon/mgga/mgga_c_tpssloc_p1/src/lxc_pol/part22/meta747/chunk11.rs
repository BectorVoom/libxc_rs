//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2499/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2499(t1070: f64, t193: f64, t336: f64, t69335: f64, t69337: f64, t69340: f64, t69343: f64, t69346: f64, t69350: f64, t69353: f64, t69357: f64, t69469: f64, t69471: f64, t69860: f64, t70985: f64, t71015: f64, t71049: f64) -> f64 {
    let t71055 = t69335 - t69337 - t69340 - t69343 - t69346 + t69350 + t69353 + t69357 - t69469 - t69471 + t193 * t336 * (t69860 + t70985 + t71015 + t71049) * t1070;
    t71055
}
