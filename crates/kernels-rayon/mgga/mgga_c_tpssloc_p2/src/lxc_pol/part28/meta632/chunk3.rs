//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1992/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1992(t87796: f64, t87804: f64, t13071: f64, t13460: f64, t2053: f64, t2054: f64, t24305: f64, t24330: f64, t25168: f64, t2597: f64, t26700: f64, t26703: f64, t26713: f64, t26728: f64, t2718: f64, t2720: f64, t4268: f64, t4273: f64, t46452: f64, t82230: f64, t82236: f64, t855: f64, t87822: f64) -> (f64, f64, f64) {
    let t92872 = 0.76763589786250567036e-1_f64 * t87796;
    let t92874 = 0.76763589786250567036e-1_f64 * t87804;
    let t92907 = 2.0_f64 * t26713 * t2720 - 0.76763589786250567036e-1_f64 * t82230 + 2.0_f64 * t4268 * t24330 - 0.3289868133696452873e-1_f64 * t87822 - 0.82246703342411321825e-2_f64 * t82236 - 12.0_f64 * t25168 * t26728 * t13071 + 4.0_f64 * t24305 * t4273 + 2.0_f64 * t855 * t2718 * t2053 * t13460 + 2.0_f64 * t26700 * t2720 + 4.0_f64 * t2597 * t26703 - t46452 * t2054;
    (t92872, t92874, t92907)
}
