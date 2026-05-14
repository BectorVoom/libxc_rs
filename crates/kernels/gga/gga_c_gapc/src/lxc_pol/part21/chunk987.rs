//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 987/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk987<F: Float>(t7113: F, t7547: F, t7549: F, t33956: F, t33962: F, t33967: F, t33969: F, t33972: F, t33975: F, t33978: F, t33980: F, t33983: F, t33988: F, t1882: F, t277: F, t9959: F) -> (F, F) {
    let t33991 = t7547 * t7113 * t7549;
    let t33993 = -0.33701061062674031276e-7 * t33956 - 0.10020915386217878654e-6 * t33962 + 0.41822872250168411824e-8 * t33967 - 0.12650553385416666667e-5 * t33969 + 0.11594181388521408695e-4 * t33972 - 0.35848176214430067278e-9 * t33975 + 0.23898784142953378185e-9 * t33978 + 0.57970906942607043474e-5 * t33980 - 0.13656448081687644677e-9 * t33983 - 0.24877751768706223874e-6 * t33988 - 0.91551759647971344971e-6 * t33991;
    let t33998 = t277 * t1882 * t9959;
    (t33993, t33998)
}
