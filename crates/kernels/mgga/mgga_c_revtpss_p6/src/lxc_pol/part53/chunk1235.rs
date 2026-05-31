//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1235/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1235<F: Float>(t127384: F, t127385: F, t127393: F, t127395: F, t127397: F, t127399: F, t127401: F, t127403: F, t127405: F, t1843: F, t1932: F, t29337: F, t32815: F, t5517: F, t6983: F, t8233: F, t8741: F) -> F {
    let t129519 = -t1843 * t32815 - t1932 * t29337 - t5517 * t8741 - t6983 * t8233 - t127384 - t127385 - F::cast_from(2.0_f64) * t127393 - F::cast_from(2.0_f64) * t127395 - F::cast_from(2.0_f64) * t127397 - F::cast_from(2.0_f64) * t127399 - F::cast_from(2.0_f64) * t127401 - F::cast_from(2.0_f64) * t127403 - F::cast_from(2.0_f64) * t127405;
    t129519
}
