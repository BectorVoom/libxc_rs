//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3637/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3637<F: Float>(t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68312: F, t68315: F, t68319: F, t68322: F, t68326: F, t68330: F, t68332: F, t68334: F, t68336: F) -> F {
    let t68837 = -F::cast_from(0.19931111111111111111e0_f64) * t68287 - F::cast_from(0.11958666666666666667e1_f64) * t68292 + F::cast_from(0.11958666666666666667e1_f64) * t68297 + F::cast_from(0.59793333333333333334e0_f64) * t68301 + F::new(0.17938e1) * t68305 - F::cast_from(0.88582716049382716048e0_f64) * t68310 + F::cast_from(0.36514074074074074074e-1_f64) * t68312 + F::cast_from(0.32862666666666666666e0_f64) * t68315 + F::cast_from(0.49293999999999999999e0_f64) * t68319 + F::new(0.197176e1) * t68322 - F::cast_from(0.16431333333333333333e0_f64) * t68326 - F::cast_from(0.98587999999999999998e0_f64) * t68330 + F::cast_from(0.13287407407407407408e0_f64) * t68332 + F::cast_from(0.26574814814814814814e0_f64) * t68334 + F::cast_from(0.79724444444444444445e0_f64) * t68336;
    t68837
}
