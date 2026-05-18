//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 415/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk415<F: Float>(t239: F, t64: F, t2270: F, t2276: F, t2279: F, t2283: F, t2286: F, t44: F, t49: F, t56: F, t614: F, t617: F) -> (F, F) {
    let t2289 = t64 * t239;
    let t2290 = F::new(88.0) / F::new(9.0) * t2289;
    let t2291 = F::new(88.0) / F::new(9.0) * t2270 * t49 - F::new(40.0) / F::new(9.0) * t614 * t617 + F::new(5.0) / F::new(18.0) * t44 * t2276 + F::new(5.0) / F::new(6.0) * t44 * t2279 + F::new(5.0) / F::new(18.0) * t56 * t2283 - F::new(5.0) / F::new(6.0) * t56 * t2286 - t2290;
    (t2289, t2291)
}
