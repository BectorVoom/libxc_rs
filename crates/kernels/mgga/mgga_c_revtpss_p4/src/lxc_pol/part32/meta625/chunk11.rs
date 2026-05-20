//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1989/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1989<F: Float>(t102512: F, t102516: F, t102518: F, t108583: F, t108587: F, t96321: F, t96322: F, t96323: F, t98200: F, t98217: F, t98218: F, t98220: F) -> F {
    let t109816 = -t102512 - t96321 + F::cast_from(0.81312004494856525159e-4_f64) * t98200 + t96322 + t102516 - F::cast_from(0.85748036236139473944e-3_f64) * t108583 + t102518 - t96323 + t98217 - F::cast_from(0.243905525293907837e-2_f64) * t98218 + F::cast_from(0.11433071498151929859e-3_f64) * t108587 - F::cast_from(0.36143185997963725434e-4_f64) * t98220;
    t109816
}
