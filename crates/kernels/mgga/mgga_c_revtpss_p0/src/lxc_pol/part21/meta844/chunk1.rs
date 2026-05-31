//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3158/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3158<F: Float>(t2439: F, t5098: F, t56248: F, t56252: F, t56256: F, t58202: F, t58207: F, t58209: F, t58211: F, t58214: F, t58217: F, t58220: F, t58223: F) -> (F, F) {
    let t58225 = t2439 * t5098;
    let t58226 = F::cast_from(0.69463333333333333334e0_f64) * t58225;
    let t58227 = F::cast_from(0.62517e0_f64) * t58202 + F::cast_from(0.17215833333333333333e1_f64) * t56248 + F::cast_from(0.929655e1_f64) * t56252 - F::cast_from(0.61977e1_f64) * t56256 - F::cast_from(0.92617777777777777778e-1_f64) * t58207 - F::cast_from(0.41678000000000000001e0_f64) * t58209 - F::cast_from(0.125034e1_f64) * t58211 + F::cast_from(0.55570666666666666666e0_f64) * t58214 + F::cast_from(0.20839e0_f64) * t58217 + F::cast_from(0.187551e1_f64) * t58220 + F::cast_from(0.250068e1_f64) * t58223 + t58226;
    (t58225, t58227)
}
