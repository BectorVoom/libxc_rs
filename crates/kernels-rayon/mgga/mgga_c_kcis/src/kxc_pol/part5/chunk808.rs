//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 808/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk808(t26: f64, t6386: f64, t2955: f64, t2967: f64, t4612: f64, t4706: f64, t6328: f64, t6332: f64, t6336: f64, t6341: f64, t6343: f64, t6375: f64, t6377: f64, t6381: f64, t6384: f64) -> (f64, f64) {
    let t6387 = t26 * t6386;
    let t6389 = -0.9494625e0_f64 * t6341 + 0.1898925e1_f64 * t6343 + t2955 + 0.19931111111111111111e0_f64 * t4612 - 0.19931111111111111111e0_f64 * t6328 + 0.59793333333333333334e0_f64 * t6332 - 0.29896666666666666667e0_f64 * t6336 + 0.15358125e0_f64 * t6375 + 0.3071625e0_f64 * t6377 + t2967 + 0.10954222222222222222e0_f64 * t4706 - 0.27385555555555555556e-1_f64 * t6381 + 0.16431333333333333333e0_f64 * t6384 - 0.82156666666666666667e-1_f64 * t6387;
    (t6387, t6389)
}
