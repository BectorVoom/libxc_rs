//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2014/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2014<F: Float>(t99041: F, t99044: F, t99050: F, t93001: F, t95673: F, t95674: F, t95675: F, t99046: F, t99048: F, t99052: F, t99054: F, t99056: F) -> F {
    let t103301 = F::cast_from(0.22866142996303859718e-3_f64) * t99041;
    let t103302 = F::cast_from(0.40656002247428262579e-4_f64) * t99044;
    let t103305 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t99050;
    let t103310 = t103301 + t103302 + t99046 / F::cast_from(4.0_f64) + t99048 / F::cast_from(8.0_f64) - t95673 - t103305 + F::cast_from(0.34299214494455789578e-2_f64) * t99052 + F::cast_from(0.68598428988911579156e-2_f64) * t99054 + F::cast_from(0.51448821741683684367e-2_f64) * t99056 - t95674 + t95675 - F::cast_from(0.24390552529390783699e-2_f64) * t93001;
    t103310
}
