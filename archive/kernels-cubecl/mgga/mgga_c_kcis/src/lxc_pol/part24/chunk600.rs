//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 600/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk600<F: Float>(t3005: F, t6406: F, t971: F, t3013: F, t3020: F, t4612: F, t4706: F, t6328: F, t6332: F, t6336: F, t6341: F, t6343: F, t6375: F, t6377: F, t6381: F, t6384: F, t6387: F) -> (F, F) {
    let t6408 = t3005 * t6406 * t971;
    let t6423 = -F::cast_from(0.1294625e1_f64) * t6341 + F::cast_from(0.258925e1_f64) * t6343 + t3013 + F::cast_from(0.20128333333333333334e0_f64) * t4612 - F::cast_from(0.20128333333333333333e0_f64) * t6328 + F::cast_from(0.60385e0_f64) * t6332 - F::cast_from(0.301925e0_f64) * t6336 + F::cast_from(0.82524375e-1_f64) * t6375 + F::cast_from(0.16504875e0_f64) * t6377 + t3020 + F::cast_from(0.11038e0_f64) * t4706 - F::cast_from(0.27595e-1_f64) * t6381 + F::cast_from(0.16557e0_f64) * t6384 - F::cast_from(0.82785e-1_f64) * t6387;
    (t6408, t6423)
}
