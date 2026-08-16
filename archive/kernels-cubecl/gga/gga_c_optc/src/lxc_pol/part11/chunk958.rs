//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 958/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk958<F: Float>(t11677: F, t14881: F, t14883: F, t14895: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t17412: F, t17597: F) -> F {
    let t17609 = -F::cast_from(0.80768518518518518518e3_f64) * t17338 - F::cast_from(0.72691666666666666667e3_f64) * t17358 + F::cast_from(0.43614999999999999999e4_f64) * t17354 + F::cast_from(0.29076666666666666666e4_f64) * t17342 - F::cast_from(0.14538333333333333333e4_f64) * t17346 - F::cast_from(0.43614999999999999999e4_f64) * t17350 - F::cast_from(0.34962962962962962963e2_f64) * t17412 - F::cast_from(0.26222222222222222223e3_f64) * t11677 + F::cast_from(0.52444444444444444444e2_f64) * t14895 - F::cast_from(0.31466666666666666667e3_f64) * t14881 + F::cast_from(0.15733333333333333334e3_f64) * t14883;
    let t17610 = t17597 + t17609;
    t17610
}
