//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 250/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk250(t875: f64, t966: f64, t330: f64, t197: f64, t321: f64, t322: f64, t326: f64, t334: f64, t886: f64, t890: f64, t893: f64, t898: f64, t899: f64, t904: f64, t907: f64, t910: f64, t913: f64, t917: f64, t920: f64, t925: f64, t929: f64, t934: f64, t937: f64, t940: f64, t943: f64, t946: f64, t949: f64, t954: f64, t957: f64, t962: f64) -> (f64, f64, f64, f64) {
    let t967 = t966 * t875;
    let t968 = t330 * t967;
    let t969 = t197 * t968;
    let t972 = 0.13900948042322754167e-2_f64 * t886 * t322 - 0.13900948042322754167e-2_f64 * t890 * t893 - 0.34752370105806885418e-4_f64 * t898 * t899 + 0.61789714048124642274e-4_f64 * t904 * t907 - 0.3243554543208642639e-2_f64 * t321 * t910 + 0.13900948042322754167e-2_f64 * t321 * t913 + 0.20272215895054016493e-3_f64 * t917 * t920 - 0.13900948042322754167e-2_f64 * t321 * t925 - 0.57970906942607043474e-5_f64 * t929 * t334 + 0.57970906942607043474e-5_f64 * t934 * t937 + 0.96618178237678405792e-7_f64 * t940 * t943 - 0.1717871209065922055e-6_f64 * t946 * t943 + 0.27053089906549953621e-4_f64 * t326 * t949 - 0.11594181388521408695e-4_f64 * t326 * t954 - 0.16908181191593721013e-5_f64 * t957 * t962 + 0.11594181388521408695e-4_f64 * t326 * t969;
    (t967, t968, t969, t972)
}
