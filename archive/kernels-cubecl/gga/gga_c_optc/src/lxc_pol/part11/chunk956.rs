//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 956/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk956<F: Float>(t11671: F, t14885: F, t14887: F, t14889: F, t17389: F, t17392: F, t17406: F, t17409: F, t17419: F, t9268: F, t9269: F, t11677: F, t14881: F, t14883: F, t14895: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t17412: F) -> (F, F) {
    let t17561 = -F::cast_from(0.2585111111111111111e1_f64) * t11671 - F::cast_from(0.12315e-2_f64) * t17419 - t9268 - t9269 - F::cast_from(0.38776666666666666665e1_f64) * t14887 + F::cast_from(0.19388333333333333333e1_f64) * t14889 + F::cast_from(0.12925555555555555555e1_f64) * t14885 + F::cast_from(0.2463e-2_f64) * t17406 - F::cast_from(0.12315e-2_f64) * t17389 - F::cast_from(0.7389e-2_f64) * t17409 + F::cast_from(0.7389e-2_f64) * t17392;
    let t17573 = -F::cast_from(0.21542592592592592592e1_f64) * t17338 - F::cast_from(0.19388333333333333333e1_f64) * t17358 + F::cast_from(0.11633e2_f64) * t17354 + F::cast_from(0.77553333333333333331e1_f64) * t17342 - F::cast_from(0.38776666666666666665e1_f64) * t17346 - F::cast_from(0.11633e2_f64) * t17350 - F::cast_from(0.54733333333333333333e-3_f64) * t17412 - F::cast_from(0.4105e-2_f64) * t11677 + F::cast_from(0.821e-3_f64) * t14895 - F::cast_from(0.4926e-2_f64) * t14881 + F::cast_from(0.2463e-2_f64) * t14883;
    (t17561, t17573)
}
