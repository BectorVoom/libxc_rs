//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2268/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2268<F: Float>(t109172: F, t109176: F, t109178: F, t109180: F, t109182: F, t109194: F, t109196: F, t109198: F, t109202: F, t109262: F, t109266: F, t109268: F, t109271: F, t109274: F, t111809: F, t113002: F, t118: F, t18220: F, t2163: F, t29427: F, t4297: F, t5884: F, t6934: F, t7683: F, t7687: F) -> F {
    let t113012 = -t109172 - t118 * (t111809 + t113002) - F::cast_from(2.0_f64) * t18220 * t2163 - F::cast_from(2.0_f64) * t5884 * t7683 + t109176 - t109178 + t109180 + t7687 * t6934 + t109182 - F::cast_from(4.0_f64) * t29427 * t4297 - t109194 - t109196 - t109198 + t109202 + t109262 + t109266 - t109268 + t109271 - t109274;
    t113012
}
