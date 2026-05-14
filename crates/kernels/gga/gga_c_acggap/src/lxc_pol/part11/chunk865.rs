//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 865/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk865<F: Float>(t3445: F, t7647: F, t377: F, t7732: F, t947: F, t31404: F, t7507: F, t7517: F, t31491: F, t7381: F, t922: F, t2020: F, t7855: F, t3088: F, t7646: F, t3453: F) -> (F, F, F, F, F, F, F) {
    let t31859 = t7647 * t3445;
    let t31863 = t377 * t7732;
    let t31864 = t31863 * t947;
    let t31867 = t7507 * t31404 * t7517;
    let t31868 = 0.1383716060742582691e-1 * t31867;
    let t31870 = t31491 * t7381 * t922;
    let t31872 = t2020 * t7855;
    let t31878 = t3088 * t7646;
    let t31879 = t31878 * t3453;
    (t31859, t31864, t31868, t31870, t31872, t31878, t31879)
}
