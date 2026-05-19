//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 978/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk978<F: Float>(t1480: F, t535: F, t551: F, t6045: F, t1371: F, t6046: F, t16465: F, t125: F, t143: F, t1503: F, t1593: F, t16397: F, t16404: F, t16407: F, t16411: F, t16415: F, t16418: F, t16422: F, t16423: F, t16428: F, t16432: F, t16436: F, t18040: F, t18097: F, t2026: F, t523: F, t5598: F, t5650: F, t5653: F, t5657: F, t5680: F, t5888: F, t8335: F) -> F {
    let t18102 = t6045 * t535 * t551 * t1480;
    let t18106 = F::cast_from(0.65586876954174354395e-3_f64) * t6046 * t1371 * t1480;
    let t18108 = F::cast_from(0.12955432484775181115e-2_f64) * t16465 * t1480;
    let t18113 = F::new(18.0) * t5598 * t5657 - F::new(6.0) * t523 * t16397 + F::new(24.0) * t5598 * t5680 - F::cast_from(0.10931146159029059066e-3_f64) * t16404 - F::cast_from(0.21862292318058118132e-3_f64) * t16407 - F::cast_from(0.18276876377896586758e-4_f64) * t16411 - t16415 - t16418 + t16422 + F::new(18.0) * t1503 * t143 * t16423 - F::new(18.0) * t5650 * t16428 - F::new(3.0) * t523 * t16432 - F::new(24.0) * t16436 * t5653 + (t18040 + t18097) * t125 + F::cast_from(0.65586876954174354395e-3_f64) * t18102 + t18106 - t18108 + F::new(3.0) * t1593 * t5888 + F::new(36.0) * t8335 * t2026;
    t18113
}
