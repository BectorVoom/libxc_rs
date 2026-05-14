//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 972/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk972<F: Float>(t26373: F, t6100: F, t4129: F, t6119: F, t6124: F, t13715: F, t7976: F, t6125: F, t1264: F, t1276: F, t13588: F, t13680: F, t20552: F, t20567: F, t26336: F, t26341: F, t26345: F, t26348: F, t26352: F, t26359: F, t26362: F, t26365: F, t26368: F, t4031: F, t4081: F, t4096: F, t6040: F, t6083: F, t6095: F, t6102: F, t6126: F, t7995: F) -> (F,) {
    let t26374 = t6100 * t26373;
    let t26377 = t4129 * t6119;
    let t26378 = t6124 * t26377;
    let t26381 = t13715 * t7976;
    let t26382 = t26381 * t6125;
    let t26385 = -0.58482233974552040708e0 * t4096 * t7995 - 0.35089340384731224426e1 * t1264 * t26336 - 0.346315117987517266e2 * t6095 * t6126 - 0.58482233974552040708e0 * t26341 * t1276 + 0.16081824322151104822e2 * t4081 * t26345 + 0.32163648644302209644e2 * t4081 * t26348 + 0.51725014705706168417e3 * t13680 * t26352 - 4.0 * t20552 * t6040 + 0.32163648644302209644e2 * t20567 * t6083 + 6.0 * t4081 * t26359 - 4.0 * t4031 * t26362 - 0.96490945932906628932e2 * t13588 * t26365 - 2.0 * t4031 * t26368 + 0.23392893589820816284e1 * t6095 * t6102 + 0.23392893589820816284e1 * t1264 * t26374 - 0.34631511798751726598e2 * t1264 * t26378 + 0.1038945353962551798e3 * t1264 * t26382;
    (t26385,)
}
