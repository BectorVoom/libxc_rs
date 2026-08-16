//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 929/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk929<F: Float>(t73688: F, t73701: F, t73704: F, t3351: F, t498: F, t7231: F, t875: F, t9551: F, t3352: F, t9568: F, t3219: F, t38638: F) -> (F, F, F, F, F, F) {
    let t76604 = F::cast_from(0.5959043985061697516e-4_f64) * t73688;
    let t76607 = F::cast_from(0.2627895913935205078e-5_f64) * t73701;
    let t76608 = F::cast_from(0.59127658063542114255e-5_f64) * t73704;
    let t76612 = t3351 * t7231 * t875 * t9551 * t498;
    let t76613 = F::cast_from(0.85129199786595678796e-5_f64) * t76612;
    let t76616 = t3351 * t3352 * t875 * t9568;
    let t76617 = F::cast_from(0.25538759935978703639e-4_f64) * t76616;
    let t76618 = t38638 * t3219;
    (t76604, t76607, t76608, t76613, t76617, t76618)
}
