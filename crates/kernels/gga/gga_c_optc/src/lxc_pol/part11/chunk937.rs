//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 937/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk937<F: Float>(t11671: F, t14885: F, t14887: F, t14889: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t8857: F, t415: F) -> (F, F) {
    let t17360 = -t8857 - F::cast_from(0.12361111111111111111e-1_f64) * t11671 + F::cast_from(0.61805555555555555556e-2_f64) * t14885 - F::cast_from(0.18541666666666666667e-1_f64) * t14887 + F::cast_from(0.92708333333333333334e-2_f64) * t14889 - F::cast_from(0.10300925925925925926e-1_f64) * t17338 + F::cast_from(0.37083333333333333333e-1_f64) * t17342 - F::cast_from(0.18541666666666666666e-1_f64) * t17346 - F::cast_from(0.55625000000000000001e-1_f64) * t17350 + F::cast_from(0.55625000000000000001e-1_f64) * t17354 - F::cast_from(0.92708333333333333333e-2_f64) * t17358;
    let t17361 = t17360 * t415;
    (t17360, t17361)
}
