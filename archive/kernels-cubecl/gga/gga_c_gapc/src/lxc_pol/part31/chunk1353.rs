//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1353/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1353<F: Float>(t1125: F, t24004: F, t12055: F, t4915: F, t687: F, t10099: F, t11155: F, t1049: F, t10786: F, t1616: F, t10526: F, t3179: F) -> (F, F, F, F, F) {
    let t36272 = t24004 * t1125;
    let t36275 = F::cast_from(12.0_f64) * t4915 * t12055 * t687;
    let t36280 = F::cast_from(2.0_f64) * t10099 * t11155;
    let t36283 = F::cast_from(2.0_f64) * t1616 * t10786 * t1049;
    let t36285 = F::cast_from(2.0_f64) * t10526 * t3179;
    (t36272, t36275, t36280, t36283, t36285)
}
