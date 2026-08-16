//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1027/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1027<F: Float>(t22724: F, t31569: F, t1985: F, t214: F, t225: F, t24063: F, t567: F, t31589: F, t6897: F, t794: F, t114297: F, t114300: F, t114317: F, t12021: F, t12030: F, t12444: F, t1323: F, t1375: F, t2016: F, t22656: F, t24082: F, t24147: F, t31555: F, t31564: F, t31584: F, t3758: F, t3882: F, t3887: F, t3888: F, t568: F, t6958: F, t6963: F, t6992: F, t7199: F, t7213: F, t84433: F, t8627: F, t8636: F) -> F {
    let t115629 = t22724 * t31569;
    let t115630 = F::cast_from(0.26044789391763585244e-1_f64) * t115629;
    let t115638 = t1985 * t214 * t24063 * t225 * t567;
    let t115658 = t6897 * t794 * t31589;
    let t115660 = t114297 + F::cast_from(4.0_f64) * t6958 * t24147 - t114300 + F::cast_from(4.0_f64) * t24082 * t6963 + F::cast_from(4.0_f64) * t3758 * t31564 + t115630 - F::cast_from(6.0_f64) * t1375 * t12021 * t8636 * t3888 + F::cast_from(0.82246703342411321825e-2_f64) * t115638 + F::cast_from(4.0_f64) * t3882 * t31555 + t114317 - F::cast_from(2.0_f64) * t84433 * t2016 + F::cast_from(2.0_f64) * t1323 * t31584 * t568 + F::cast_from(2.0_f64) * t12030 * t8627 + F::cast_from(4.0_f64) * t12444 * t8627 + F::cast_from(4.0_f64) * t22656 * t7199 + F::cast_from(4.0_f64) * t1375 * t3887 * t7213 * t6992 - F::cast_from(0.82246703342411321824e-2_f64) * t115658;
    t115660
}
