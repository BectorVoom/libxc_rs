//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 938/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk938<F: Float>(t17361: F, t241: F, t11671: F, t14885: F, t14887: F, t14889: F, t17338: F, t17342: F, t17346: F, t17350: F, t17354: F, t17358: F, t8662: F) -> (F, F) {
    let t17363 = F::cast_from(0.19751789702565206229e-1_f64) * t241 * t17361;
    let t17380 = -t8662 - F::new(4.0) / F::new(9.0) * t11671 + F::new(2.0) / F::new(9.0) * t14885 - F::new(2.0) / F::new(3.0) * t14887 + t14889 / F::new(3.0) - F::new(10.0) / F::new(27.0) * t17338 + F::new(4.0) / F::new(3.0) * t17342 - F::new(2.0) / F::new(3.0) * t17346 - F::new(2.0) * t17350 + F::new(2.0) * t17354 - t17358 / F::new(3.0);
    (t17363, t17380)
}
