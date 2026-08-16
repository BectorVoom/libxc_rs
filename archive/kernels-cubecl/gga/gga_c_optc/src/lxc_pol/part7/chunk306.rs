//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 306/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk306<F: Float>(t339: F, t765: F, t792: F, t772: F, t796: F, t349: F, t346: F) -> (F, F, F, F, F, F) {
    let t962 = F::cast_from(1.0_f64) / t339;
    let t966 = F::cast_from(0.19388333333333333333e1_f64) * t765;
    let t968 = F::cast_from(0.12315e-2_f64) * t792;
    let t970 = -t966 - F::cast_from(0.19388333333333333333e1_f64) * t772 - t968 - F::cast_from(0.12315e-2_f64) * t796;
    let t972 = t349 * t349;
    let t973 = F::cast_from(1.0_f64) / t972;
    let t974 = t346 * t973;
    let t975 = F::cast_from(0.72691666666666666667e3_f64) * t765;
    let t977 = F::cast_from(0.78666666666666666667e2_f64) * t792;
    let t979 = -t975 - F::cast_from(0.72691666666666666667e3_f64) * t772 - t977 - F::cast_from(0.78666666666666666667e2_f64) * t796;
    (t962, t970, t972, t973, t974, t979)
}
