//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2238/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2238<F: Float>(t17361: F, t7618: F, t17289: F, t2138: F, t1238: F, t16729: F, t17461: F, t17536: F, t17662: F, t26880: F, t29047: F, t29054: F, t29086: F, t3663: F, t97174: F, t97179: F, t97220: F, t97222: F, t97239: F, t97247: F) -> F {
    let t104905 = t7618 * t17361;
    let t104916 = t17289 * t2138;
    let t104921 = F::cast_from(0.57165357490759649296e-3_f64) * t26880 * t17536 - t97220 / F::new(864.0) - t97222 / F::new(432.0) - F::cast_from(0.95275595817932748827e-4_f64) * t104905 + F::cast_from(0.57165357490759649296e-3_f64) * t97174 * t17662 + F::cast_from(0.17149607247227894789e-2_f64) * t97179 * t17461 - F::cast_from(0.3811023832717309953e-3_f64) * t97239 + t29047 * t29054 * t16729 / F::new(216.0) - F::cast_from(0.19055119163586549765e-3_f64) * t97247 - F::cast_from(0.85748036236139473944e-3_f64) * t104916 * t1238 - F::cast_from(0.42874018118069736972e-3_f64) * t29086 * t3663;
    t104921
}
