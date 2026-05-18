//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1131/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1131<F: Float>(t33258: F, t3698: F, t3780: F, t15853: F, t17874: F, t311: F, t4043: F, t519: F, t7113: F, t7547: F, t7549: F, t33956: F, t33962: F, t33967: F, t33969: F, t33972: F, t33975: F, t33978: F, t33980: F) -> F {
    let t33983 = t33258 * t3698 * t3780;
    let t33988 = t311 * t15853 * t4043 * t519 * t17874;
    let t33991 = t7547 * t7113 * t7549;
    let t33993 = -F::new(0.33701061062674031276e-7) * t33956 - F::new(0.10020915386217878654e-6) * t33962 + F::new(0.41822872250168411824e-8) * t33967 - F::new(0.12650553385416666667e-5) * t33969 + F::new(0.11594181388521408695e-4) * t33972 - F::new(0.35848176214430067278e-9) * t33975 + F::new(0.23898784142953378185e-9) * t33978 + F::new(0.57970906942607043474e-5) * t33980 - F::new(0.13656448081687644677e-9) * t33983 - F::new(0.24877751768706223874e-6) * t33988 - F::new(0.91551759647971344971e-6) * t33991;
    t33993
}
