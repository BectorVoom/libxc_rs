//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2249/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2249<F: Float>(t23384: F, t25798: F, t225: F, t25822: F, t7557: F, t82632: F, t10160: F, t1066: F, t14555: F, t1599: F, t1635: F, t23346: F, t23353: F, t23365: F, t23378: F, t25403: F, t25453: F, t25738: F, t3169: F, t4557: F, t6687: F, t6816: F, t7600: F, t82442: F, t82499: F, t83457: F, t83459: F) -> F {
    let t89662 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25798;
    let t89666 = t25822 * t225;
    let t89672 = t82632 * t7557;
    let t89690 = F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25798 - t89662 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t23365 * t25738 - F::cast_from(2.0_f64) * t89666 * t1066 + F::cast_from(4.0_f64) * t3169 * t25453 - t82499 * t1635 + F::cast_from(0.18277045187202515961e-2_f64) * t89672 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25403 - F::cast_from(0.27415567780803773942e-2_f64) * t83457 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t1599 * t23353 - F::cast_from(2.0_f64) * t14555 * t6816 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t82442 + F::cast_from(4.0_f64) * t10160 * t7600 + F::cast_from(2.0_f64) * t4557 * t23378 + F::cast_from(0.36554090374405031922e-2_f64) * t83459;
    t89690
}
