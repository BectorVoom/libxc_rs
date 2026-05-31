//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3171/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3171<F: Float>(t1180: F, t1188: F, t12494: F, t12497: F, t17097: F, t17151: F, t3454: F, t3480: F, t3491: F, t58317: F, t58322: F, t58325: F, t58327: F, t58330: F, t58333: F, t58336: F, t58341: F, t58344: F, t58345: F, t58456: F, t58462: F, t58464: F) -> F {
    let t58465 = F::cast_from(0.96491876992155210402e2_f64) * t58317 * t3480 + t58322 - t58325 - t58327 - t58330 - t58333 + F::cast_from(0.35089341735807877242e1_f64) * t17097 * t12494 - F::cast_from(6.0_f64) * t58336 * t3454 + t58341 + t58344 + F::cast_from(0.10526802520742363173e2_f64) * t58345 * t12497 + F::cast_from(0.17544670867903938621e1_f64) * t3491 * t17151 + F::cast_from(0.5848223622634646207e0_f64) * t1180 * t58456 * t1188 - t58462 - t58464;
    t58465
}
