//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 993/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk993<F: Float>(t13565: F, t30396: F, t1234: F, t1264: F, t13588: F, t13680: F, t20567: F, t2129: F, t26095: F, t30308: F, t30319: F, t30381: F, t30384: F, t30388: F, t30393: F, t374: F, t45: F, t6035: F, t6095: F, t7960: F, t7963: F, t7999: F) -> F {
    let t30397 = t30396 * t13565;
    let t30402 = F::cast_from(0.19751789702565206229e-1_f64) * t45 * t30308 * t374 + F::new(3.0) * t26095 * t2129 + F::new(3.0) * t6035 * t7960 + F::cast_from(0.48245472966453314466e2_f64) * t20567 * t7963 - F::cast_from(0.96490945932906628932e2_f64) * t13588 * t30319 + F::new(1.0) * t1234 * t30381 + F::cast_from(0.51725014705706168417e3_f64) * t13680 * t30384 + F::cast_from(0.35089340384731224426e1_f64) * t1264 * t30388 - F::cast_from(0.35089340384731224426e1_f64) * t1264 * t30393 - F::cast_from(0.1025389702100779493e4_f64) * t1264 * t30397 - F::cast_from(0.51947267698127589899e2_f64) * t6095 * t7999;
    t30402
}
