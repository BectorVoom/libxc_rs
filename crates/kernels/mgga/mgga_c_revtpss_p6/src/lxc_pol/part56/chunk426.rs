//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 426/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk426<F: Float>(t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F, t99: F, t107: F, t200: F, t202: F, t705: F, t716: F) -> (F, F, F, F, F, F, F, F) {
    let t2335 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t2289 * t112;
    let t2336 = t625 * t666;
    let t2339 = F::cast_from(1.0_f64) / t654 / t111;
    let t2349 = F::cast_from(1.0_f64) / t99;
    let t2357 = F::cast_from(1.0_f64) / t107;
    let t2375 = F::cast_from(1.0_f64) / t200;
    let t2382 = F::cast_from(1.0_f64) / t202;
    let t2398 = t705 * t716;
    (t2335, t2336, t2339, t2349, t2357, t2375, t2382, t2398)
}
