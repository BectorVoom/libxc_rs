//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2249/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2249(t23384: f64, t25798: f64, t225: f64, t25822: f64, t7557: f64, t82632: f64, t10160: f64, t1066: f64, t14555: f64, t1599: f64, t1635: f64, t23346: f64, t23353: f64, t23365: f64, t23378: f64, t25403: f64, t25453: f64, t25738: f64, t3169: f64, t4557: f64, t6687: f64, t6816: f64, t7600: f64, t82442: f64, t82499: f64, t83457: f64, t83459: f64) -> f64 {
    let t89662 = 0.54831135561607547884e-2_f64 * t23384 * t25798;
    let t89666 = t25822 * t225;
    let t89672 = t82632 * t7557;
    let t89690 = 0.43864908449286038306e-1_f64 * t23346 * t25798 - t89662 + 0.3289868133696452873e-1_f64 * t6687 * t23365 * t25738 - 2.0_f64 * t89666 * t1066 + 4.0_f64 * t3169 * t25453 - t82499 * t1635 + 0.18277045187202515961e-2_f64 * t89672 + 0.43864908449286038306e-1_f64 * t23346 * t25403 - 0.27415567780803773942e-2_f64 * t83457 - 0.82246703342411321825e-2_f64 * t6687 * t1599 * t23353 - 2.0_f64 * t14555 * t6816 - 0.16449340668482264365e-1_f64 * t6687 * t1599 * t82442 + 4.0_f64 * t10160 * t7600 + 2.0_f64 * t4557 * t23378 + 0.36554090374405031922e-2_f64 * t83459;
    t89690
}
