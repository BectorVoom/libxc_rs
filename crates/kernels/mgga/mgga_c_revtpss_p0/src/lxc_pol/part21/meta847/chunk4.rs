//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3179/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3179<F: Float>(t56248: F, t56252: F, t56256: F, t58202: F, t58207: F, t58209: F, t58211: F, t58214: F, t58217: F, t58220: F, t58223: F, t58225: F) -> F {
    let t58585 = F::cast_from(0.49293999999999999999e0_f64) * t58202 + F::cast_from(0.99655555555555555554e0_f64) * t56248 + F::new(0.53814e1) * t56252 - F::new(0.35876e1) * t56256 - F::cast_from(0.73028148148148148149e-1_f64) * t58207 - F::cast_from(0.32862666666666666666e0_f64) * t58209 - F::cast_from(0.98587999999999999998e0_f64) * t58211 + F::cast_from(0.43816888888888888889e0_f64) * t58214 + F::cast_from(0.16431333333333333333e0_f64) * t58217 + F::new(0.147882e1) * t58220 + F::new(0.197176e1) * t58223 + F::cast_from(0.5477111111111111111e0_f64) * t58225;
    t58585
}
