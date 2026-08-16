//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1131/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1131(t33258: f64, t3698: f64, t3780: f64, t15853: f64, t17874: f64, t311: f64, t4043: f64, t519: f64, t7113: f64, t7547: f64, t7549: f64, t33956: f64, t33962: f64, t33967: f64, t33969: f64, t33972: f64, t33975: f64, t33978: f64, t33980: f64) -> f64 {
    let t33983 = t33258 * t3698 * t3780;
    let t33988 = t311 * t15853 * t4043 * t519 * t17874;
    let t33991 = t7547 * t7113 * t7549;
    let t33993 = -0.33701061062674031276e-7_f64 * t33956 - 0.10020915386217878654e-6_f64 * t33962 + 0.41822872250168411824e-8_f64 * t33967 - 0.12650553385416666667e-5_f64 * t33969 + 0.11594181388521408695e-4_f64 * t33972 - 0.35848176214430067278e-9_f64 * t33975 + 0.23898784142953378185e-9_f64 * t33978 + 0.57970906942607043474e-5_f64 * t33980 - 0.13656448081687644677e-9_f64 * t33983 - 0.24877751768706223874e-6_f64 * t33988 - 0.91551759647971344971e-6_f64 * t33991;
    t33993
}
