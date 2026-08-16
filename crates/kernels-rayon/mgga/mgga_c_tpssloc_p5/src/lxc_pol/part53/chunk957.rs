//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 957/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk957(t114688: f64, t114693: f64, t225: f64, t31985: f64, t131: f64, t32248: f64, t9239: f64, t2240: f64, t32253: f64, t33: f64, t31013: f64, t8302: f64, t8308: f64, t9533: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t116686 = 0.3289868133696452873e-1_f64 * t114688;
    let t116688 = 0.25587863262083522346e0_f64 * t114693;
    let t116709 = t31985 * t225;
    let t116904 = t32248 * t131;
    let t116905 = t9239 * t116904;
    let t116909 = t2240 * t33 * t32253;
    let t116910 = t116909 * t31013;
    let t116917 = 380.0_f64 / 81.0_f64 * t8302 * t9533 * t131 * t8308;
    (t116686, t116688, t116709, t116904, t116905, t116909, t116910, t116917)
}
