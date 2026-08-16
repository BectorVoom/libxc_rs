//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 628/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk628<F: Float>(t2251: F, t2282: F, t2258: F, t60: F, t239: F, t64: F, t2270: F, t2276: F, t2279: F, t44: F, t49: F, t56: F, t614: F, t617: F) -> (F, F, F, F, F) {
    let t2283 = t2282 * t2251;
    let t2286 = t60 * t2258;
    let t2289 = t64 * t239;
    let t2290 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2289;
    let t2291 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2270 * t49 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t614 * t617 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t44 * t2276 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t2279 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t56 * t2283 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t2286 - t2290;
    (t2283, t2286, t2289, t2290, t2291)
}
