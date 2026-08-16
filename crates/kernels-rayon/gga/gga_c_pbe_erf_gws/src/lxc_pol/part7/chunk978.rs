//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 978/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk978(t1480: f64, t535: f64, t551: f64, t6045: f64, t1371: f64, t6046: f64, t16465: f64, t125: f64, t143: f64, t1503: f64, t1593: f64, t16397: f64, t16404: f64, t16407: f64, t16411: f64, t16415: f64, t16418: f64, t16422: f64, t16423: f64, t16428: f64, t16432: f64, t16436: f64, t18040: f64, t18097: f64, t2026: f64, t523: f64, t5598: f64, t5650: f64, t5653: f64, t5657: f64, t5680: f64, t5888: f64, t8335: f64) -> f64 {
    let t18102 = t6045 * t535 * t551 * t1480;
    let t18106 = 0.65586876954174354395e-3_f64 * t6046 * t1371 * t1480;
    let t18108 = 0.12955432484775181115e-2_f64 * t16465 * t1480;
    let t18113 = 18.0_f64 * t5598 * t5657 - 6.0_f64 * t523 * t16397 + 24.0_f64 * t5598 * t5680 - 0.10931146159029059066e-3_f64 * t16404 - 0.21862292318058118132e-3_f64 * t16407 - 0.18276876377896586758e-4_f64 * t16411 - t16415 - t16418 + t16422 + 18.0_f64 * t1503 * t143 * t16423 - 18.0_f64 * t5650 * t16428 - 3.0_f64 * t523 * t16432 - 24.0_f64 * t16436 * t5653 + (t18040 + t18097) * t125 + 0.65586876954174354395e-3_f64 * t18102 + t18106 - t18108 + 3.0_f64 * t1593 * t5888 + 36.0_f64 * t8335 * t2026;
    t18113
}
