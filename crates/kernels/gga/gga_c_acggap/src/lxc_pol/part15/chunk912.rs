//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 912/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk912<F: Float>(t31629: F, t31646: F, t35500: F, t7380: F, t34050: F, t2095: F, t33901: F, t33884: F, t1998: F, t4503: F, t5124: F, t7647: F, t7310: F, t8878: F, t1446: F, t7614: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35898 = 0.12862205435420921092e-1 * t31629;
    let t35904 = 0.32012600194825403606e-1 * t31646;
    let t35909 = t7380 * t35500;
    let t35911 = t7380 * t34050;
    let t35913 = t2095 * t33901;
    let t35915 = t2095 * t33884;
    let t35917 = t1998 * t4503;
    let t35919 = t7647 * t5124;
    let t35924 = t7310 * t8878;
    let t35926 = t7614 * t1446;
    (t35898, t35904, t35909, t35911, t35913, t35915, t35917, t35919, t35924, t35926)
}
