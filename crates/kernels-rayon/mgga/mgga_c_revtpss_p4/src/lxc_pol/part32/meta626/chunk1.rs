//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1991/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1991(t102557: f64, t108601: f64, t108604: f64, t108606: f64, t108608: f64, t94546: f64, t96341: f64, t96342: f64, t98263: f64, t98264: f64, t98267: f64, t98268: f64) -> f64 {
    let t109829 = -t98263 - t96341 + t96342 - t98264 + 0.28582678745379824648e-4_f64 * t108601 - 0.57165357490759649296e-4_f64 * t108604 - 0.34299214494455789578e-2_f64 * t108606 - 0.2032800112371413129e-3_f64 * t108608 + t98267 - t98268 + t102557 - 0.90702367218671976884e-1_f64 * t94546;
    t109829
}
