//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1991/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1991<F: Float>(t102557: F, t108601: F, t108604: F, t108606: F, t108608: F, t94546: F, t96341: F, t96342: F, t98263: F, t98264: F, t98267: F, t98268: F) -> F {
    let t109829 = -t98263 - t96341 + t96342 - t98264 + F::cast_from(0.28582678745379824648e-4_f64) * t108601 - F::cast_from(0.57165357490759649296e-4_f64) * t108604 - F::cast_from(0.34299214494455789578e-2_f64) * t108606 - F::cast_from(0.2032800112371413129e-3_f64) * t108608 + t98267 - t98268 + t102557 - F::cast_from(0.90702367218671976884e-1_f64) * t94546;
    t109829
}
