//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1185/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1185<F: Float>(t11889: F, t1206: F, t14327: F, t3207: F, t3913: F, t53334: F, t55031: F, t55036: F, t55059: F, t55062: F, t56299: F, t56302: F, t56305: F, t56307: F, t56309: F, t56312: F, t56316: F, t56318: F, t56321: F, t9283: F) -> (F,) {
    let t58083 = -t55031 + t56299 / 256.0 - t3207 * t9283 * t1206 * t11889 / 8.0 - t3913 * t14327 / 96.0 + t56302 / 768.0 + t56305 / 192.0 - t56307 / 24.0 - t56309 / 12.0 - 5.0 / 192.0 * t56312 - t56316 / 48.0 - t56318 / 12.0 - t56321 / 48.0 + t55036 + 35.0 / 108.0 * t55059 - t55062 - 119.0 / 3456.0 * t53334;
    (t58083,)
}
