//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2709/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2709<F: Float>(t39438: F, t1469: F, t2608: F, t4401: F, t606: F, t10428: F, t4308: F, t14425: F, t705: F, t707: F, t10356: F, t1522: F, t157: F) -> (F, F, F, F, F) {
    let t49873 = F::cast_from(0.48796115851357829289e-1_f64) * t39438;
    let t49876 = t4401 * t2608 * t1469 * t606;
    let t49877 = F::cast_from(36.0_f64) * t49876;
    let t49879 = F::cast_from(12.0_f64) * t10428 * t4308;
    let t49880 = t705 * t14425;
    let t49882 = F::cast_from(12.0_f64) * t49880 * t707;
    let t49885 = F::cast_from(24.0_f64) * t10356 * t157 * t1522;
    (t49873, t49877, t49879, t49882, t49885)
}
