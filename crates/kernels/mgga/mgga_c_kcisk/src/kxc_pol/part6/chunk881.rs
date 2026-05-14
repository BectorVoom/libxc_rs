//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 881/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk881<F: Float>(t13682: F, t30318: F, t1275: F, t7993: F, t6100: F, t2141: F, t7976: F, t4126: F, t13561: F, t13565: F, t1234: F, t1264: F, t13588: F, t13680: F, t20567: F, t2129: F, t26095: F, t30308: F, t30319: F, t30381: F, t374: F, t45: F, t6035: F, t6095: F, t7960: F, t7963: F, t7999: F) -> (F, F) {
    let t30384 = t30318 * t13682;
    let t30387 = t1275 * t7993;
    let t30388 = t6100 * t30387;
    let t30391 = t7976 * t2141;
    let t30393 = t4126 * t30391 * t1275;
    let t30396 = t13561 * t30391;
    let t30397 = t30396 * t13565;
    let t30402 = 0.19751789702565206229e-1 * t45 * t30308 * t374 + 3.0 * t26095 * t2129 + 3.0 * t6035 * t7960 + 0.48245472966453314466e2 * t20567 * t7963 - 0.96490945932906628932e2 * t13588 * t30319 + 1.0 * t1234 * t30381 + 0.51725014705706168417e3 * t13680 * t30384 + 0.35089340384731224426e1 * t1264 * t30388 - 0.35089340384731224426e1 * t1264 * t30393 - 0.1025389702100779493e4 * t1264 * t30397 - 0.51947267698127589899e2 * t6095 * t7999;
    (t30391, t30402)
}
