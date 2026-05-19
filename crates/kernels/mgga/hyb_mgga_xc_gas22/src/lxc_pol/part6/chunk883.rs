//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 883/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk883<F: Float>(t7393: F, t7481: F, t468: F, t462: F, t1025: F, t2649: F, t2630: F, t1112: F, t2662: F, t2676: F, t2640: F, t1067: F, t221: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7482 = t7393 + t7481;
    let t7483 = t468 * t7482;
    let t7484 = t462 * t7483;
    let t7485 = t1025 * t2649;
    let t7487 = F::cast_from(0.32530743900905219526e-1_f64) * t2630 * t7485;
    let t7488 = t2662 * t1112;
    let t7490 = F::cast_from(0.21687162600603479684e-1_f64) * t2630 * t7488;
    let t7491 = t1025 * t2676;
    let t7493 = F::cast_from(0.16265371950452609763e-1_f64) * t2630 * t7491;
    let t7494 = t1025 * t2640;
    let t7496 = F::cast_from(0.48159733137676571078e0_f64) * t2630 * t7494;
    let t7497 = t1067 * t221;
    (t7482, t7483, t7484, t7485, t7487, t7488, t7490, t7491, t7493, t7494, t7496, t7497)
}
