//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 644/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk644<F: Float>(t2251: F, t2258: F, t2299: F, t2306: F, t633: F, t637: F, t77: F) -> F {
    let t2311 = F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2299 * t2251 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t2258 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2306 * t2251 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t2258;
    let t2312 = t77 * t2311;
    t2312
}
