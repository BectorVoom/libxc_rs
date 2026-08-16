//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2250/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2250<F: Float>(t109172: F, t109176: F, t109178: F, t109180: F, t109182: F, t109194: F, t109196: F, t109198: F, t109202: F, t109231: F, t109258: F, t109262: F, t109266: F, t109268: F, t109271: F, t109274: F, t2322: F, t30119: F, t4254: F, t4292: F, t569: F, t5920: F, t651: F, t7221: F, t7883: F) -> F {
    let t109275 = -t109172 + t109176 - t109178 + t109180 + t109182 - F::cast_from(4.0_f64) * t651 * t7883 * t4292 - F::cast_from(2.0_f64) * t2322 * t30119 - F::cast_from(2.0_f64) * t4254 * t30119 - F::cast_from(2.0_f64) * t651 * t7221 * t5920 - t109194 - t109196 - t109198 + t109202 + (t109231 + t109258) * t569 + t109262 + t109266 - t109268 + t109271 - t109274;
    t109275
}
