//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 670/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk670<F: Float>(t1940: F, t1963: F, t2403: F, t30: F, t605: F, t7010: F, t7087: F, t7091: F, t7092: F, t1976: F, t994: F, t343: F, t613: F) -> (F, F, F) {
    let t7099 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2403 * t1963 * t7010 + t1940 * t7087 * t30 / F::cast_from(2.0_f64) - t1940 * t7091 * t7092 / F::cast_from(2.0_f64) + t1940 * t1963 * t605 / F::cast_from(2.0_f64);
    let t7102 = t994 * t1976;
    let t7105 = t613 * t343;
    (t7099, t7102, t7105)
}
