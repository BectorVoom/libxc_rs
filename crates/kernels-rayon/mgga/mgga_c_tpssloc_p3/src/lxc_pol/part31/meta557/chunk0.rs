//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1785/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1785(t82122: f64, t82153: f64, t82218: f64, t81440: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64, t22470: f64, t4067: f64, t25: f64, t40772: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t85060 = 0.3244175520728446583e0_f64 * t82122;
    let t85101 = 0.27415567780803773942e-2_f64 * t82153;
    let t85129 = 0.55440370401180965083e0_f64 * t82218;
    let t86583 = 22.0_f64 / 9.0_f64 * t81440;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    let t86590 = t22470 * t4067;
    let t86716 = t40772 * t25;
    (t85060, t85101, t85129, t86583, t86586, t86588, t86590, t86716)
}
