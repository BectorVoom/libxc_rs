//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 597/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk597<F: Float>(t26: F, t6386: F, t2955: F, t2967: F, t4612: F, t4706: F, t6328: F, t6332: F, t6336: F, t6341: F, t6343: F, t6375: F, t6377: F, t6381: F, t6384: F) -> (F, F) {
    let t6387 = t26 * t6386;
    let t6389 = -F::new(0.9494625e0) * t6341 + F::new(0.1898925e1) * t6343 + t2955 + F::cast_from(0.19931111111111111111e0_f64) * t4612 - F::cast_from(0.19931111111111111111e0_f64) * t6328 + F::cast_from(0.59793333333333333334e0_f64) * t6332 - F::cast_from(0.29896666666666666667e0_f64) * t6336 + F::new(0.15358125e0) * t6375 + F::new(0.3071625e0) * t6377 + t2967 + F::cast_from(0.10954222222222222222e0_f64) * t4706 - F::cast_from(0.27385555555555555556e-1_f64) * t6381 + F::cast_from(0.16431333333333333333e0_f64) * t6384 - F::cast_from(0.82156666666666666667e-1_f64) * t6387;
    (t6387, t6389)
}
