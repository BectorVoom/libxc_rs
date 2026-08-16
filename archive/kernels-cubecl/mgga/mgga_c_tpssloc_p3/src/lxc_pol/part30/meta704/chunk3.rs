//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2300/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2300<F: Float>(t23384: F, t28492: F, t28500: F, t1599: F, t1625: F, t18071: F, t23327: F, t23346: F, t25429: F, t25431: F, t25712: F, t28684: F, t28691: F, t343: F, t6687: F, t6690: F, t6771: F, t7553: F, t83444: F, t88050: F, t88105: F, t89630: F, t89648: F, t89653: F) -> F {
    let t99948 = t23384 * t28492;
    let t99956 = t23384 * t28500;
    let t99959 = F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t28684 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t89648 + t89630 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25712 * t343 * t1625 * t6690 + F::cast_from(0.21932454224643019153e-1_f64) * t23346 * t28691 - F::cast_from(6.0_f64) * t6771 * t18071 - F::cast_from(0.97477574331746751795e-2_f64) * t23346 * t28492 + F::cast_from(0.12184696791468343974e-2_f64) * t99948 + F::cast_from(0.73108180748810063845e-2_f64) * t25429 * t88050 * t25431 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88105 * t7553 - F::cast_from(0.18277045187202515961e-2_f64) * t99956 - F::cast_from(0.18277045187202515961e-2_f64) * t83444 - t89653;
    t99959
}
