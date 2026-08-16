//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2274/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2274<F: Float>(t225: F, t28488: F, t10164: F, t1066: F, t14545: F, t14555: F, t1599: F, t17575: F, t17588: F, t1921: F, t23365: F, t23588: F, t25757: F, t25801: F, t25810: F, t28485: F, t28495: F, t3169: F, t387: F, t4540: F, t4664: F, t5838: F, t6687: F, t6776: F, t7600: F, t7624: F, t7625: F, t88731: F, t88753: F) -> F {
    let t99248 = t28488 * t225;
    let t99271 = -F::cast_from(12.0_f64) * t25757 * t10164 * t7624 * t4664 - F::cast_from(0.12184696791468343974e-2_f64) * t88731 - F::cast_from(2.0_f64) * t14545 * t7625 + F::cast_from(4.0_f64) * t17588 * t6776 - F::cast_from(2.0_f64) * t99248 * t1066 + F::cast_from(4.0_f64) * t14555 * t7600 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t23365 * t28495 + F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t5838 * t23588 + F::cast_from(2.0_f64) * t17575 * t6776 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t1921 * t387 * t4540 + F::cast_from(4.0_f64) * t3169 * t28485 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t25810 * t25801 - t88753;
    t99271
}
