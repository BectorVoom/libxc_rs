//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 970/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk970<F: Float>(t1471: F, t15401: F, t11671: F, t14885: F, t14887: F, t14889: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t8871: F) -> (F, F) {
    let t17764 = t15401 * t1471;
    let t17777 = -t8871 - F::cast_from(0.2283111111111111111e-1_f64) * t11671 + F::cast_from(0.11415555555555555555e-1_f64) * t14885 - F::cast_from(0.34246666666666666665e-1_f64) * t14887 + F::cast_from(0.17123333333333333333e-1_f64) * t14889 - F::cast_from(0.19025925925925925925e-1_f64) * t17338 + F::cast_from(0.68493333333333333331e-1_f64) * t17342 - F::cast_from(0.34246666666666666665e-1_f64) * t17346 - F::cast_from(0.10274e0_f64) * t17350 + F::cast_from(0.10274e0_f64) * t17354 - F::cast_from(0.17123333333333333333e-1_f64) * t17358;
    (t17764, t17777)
}
