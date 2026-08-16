//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2011/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2011<F: Float>(t90642: F, t90645: F, t90659: F, t90663: F, t90686: F, t90701: F, t12021: F, t12033: F, t1375: F, t16460: F, t16475: F, t2092: F, t27062: F, t27115: F, t3758: F, t3882: F, t3887: F, t3888: F, t3911: F, t55134: F, t7194: F, t7199: F, t7925: F, t7936: F, t81264: F, t81267: F, t84423: F, t90639: F, t90690: F, t90704: F) -> F {
    let t93438 = F::cast_from(0.16449340668482264365e-1_f64) * t90642;
    let t93439 = F::cast_from(0.16449340668482264365e-1_f64) * t90645;
    let t93445 = F::cast_from(0.12793931631041761173e0_f64) * t90659;
    let t93446 = F::cast_from(0.16449340668482264365e-1_f64) * t90663;
    let t93452 = F::cast_from(0.3289868133696452873e-1_f64) * t90686;
    let t93461 = F::cast_from(0.16449340668482264365e-1_f64) * t90701;
    let t93465 = -F::cast_from(0.3289868133696452873e-1_f64) * t90639 - F::cast_from(6.0_f64) * t7194 * t16475 + t93438 + t93439 + F::cast_from(0.10417915756705434098e0_f64) * t81264 + F::cast_from(4.0_f64) * t3882 * t27062 - F::cast_from(2.0_f64) * t3758 * t27115 - t93445 - t93446 + F::cast_from(2.0_f64) * t1375 * t3887 * t7936 * t3911 + F::cast_from(0.3289868133696452873e-1_f64) * t81267 - t93452 - F::cast_from(0.16449340668482264365e-1_f64) * t90690 + t84423 + F::cast_from(4.0_f64) * t16460 * t7199 - F::cast_from(6.0_f64) * t1375 * t12021 * t7936 * t3888 - t55134 * t2092 + t93461 - F::cast_from(0.16449340668482264365e-1_f64) * t90704 + F::cast_from(2.0_f64) * t12033 * t7925;
    t93465
}
