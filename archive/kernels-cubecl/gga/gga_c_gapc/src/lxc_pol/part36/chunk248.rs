//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 248/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk248<F: Float>(t875: F, t966: F, t330: F, t197: F, t321: F, t322: F, t326: F, t334: F, t886: F, t890: F, t893: F, t898: F, t899: F, t904: F, t907: F, t910: F, t913: F, t917: F, t920: F, t925: F, t929: F, t934: F, t937: F, t940: F, t943: F, t946: F, t949: F, t954: F, t957: F, t962: F) -> (F, F, F, F) {
    let t967 = t966 * t875;
    let t968 = t330 * t967;
    let t969 = t197 * t968;
    let t972 = F::cast_from(0.13900948042322754167e-2_f64) * t886 * t322 - F::cast_from(0.13900948042322754167e-2_f64) * t890 * t893 - F::cast_from(0.34752370105806885418e-4_f64) * t898 * t899 + F::cast_from(0.61789714048124642274e-4_f64) * t904 * t907 - F::cast_from(0.3243554543208642639e-2_f64) * t321 * t910 + F::cast_from(0.13900948042322754167e-2_f64) * t321 * t913 + F::cast_from(0.20272215895054016493e-3_f64) * t917 * t920 - F::cast_from(0.13900948042322754167e-2_f64) * t321 * t925 - F::cast_from(0.57970906942607043474e-5_f64) * t929 * t334 + F::cast_from(0.57970906942607043474e-5_f64) * t934 * t937 + F::cast_from(0.96618178237678405792e-7_f64) * t940 * t943 - F::cast_from(0.1717871209065922055e-6_f64) * t946 * t943 + F::cast_from(0.27053089906549953621e-4_f64) * t326 * t949 - F::cast_from(0.11594181388521408695e-4_f64) * t326 * t954 - F::cast_from(0.16908181191593721013e-5_f64) * t957 * t962 + F::cast_from(0.11594181388521408695e-4_f64) * t326 * t969;
    (t967, t968, t969, t972)
}
