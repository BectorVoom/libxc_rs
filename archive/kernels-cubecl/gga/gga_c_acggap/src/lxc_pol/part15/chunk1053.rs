//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1053/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1053<F: Float>(t35290: F, t35301: F, t35315: F, t35317: F, t35348: F, t35379: F, t35384: F, t35387: F, t35390: F, t35392: F, t35394: F, t35396: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37476 = F::cast_from(0.42874018118069736972e-3_f64) * t35290;
    let t37479 = F::cast_from(0.31448092289604152068e-2_f64) * t35301;
    let t37484 = F::cast_from(0.12862205435420921092e-1_f64) * t35315;
    let t37485 = F::cast_from(0.34299214494455789578e-2_f64) * t35317;
    let t37498 = F::cast_from(0.14291339372689912324e-2_f64) * t35348;
    let t37519 = F::cast_from(0.62896184579208304138e-3_f64) * t35379;
    let t37522 = F::cast_from(0.61125e-1_f64) * t35384;
    let t37523 = t35387 / F::cast_from(4.0_f64);
    let t37524 = t35390 / F::cast_from(16.0_f64);
    let t37525 = F::cast_from(0.48018900292238105409e-1_f64) * t35392;
    let t37526 = F::cast_from(0.13719685797782315831e-1_f64) * t35394;
    let t37527 = F::cast_from(0.13719685797782315831e-1_f64) * t35396;
    (t37476, t37479, t37484, t37485, t37498, t37519, t37522, t37523, t37524, t37525, t37526, t37527)
}
