//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 979/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk979<F: Float>(t2061: F, t6071: F, t7071: F, t26462: F, t26468: F, t26471: F, t27228: F, t27230: F, t27256: F, t29623: F, t29627: F, t29629: F, t29631: F, t29633: F, t26450: F, t26454: F, t26457: F, t27240: F, t27246: F, t27251: F, t27254: F, t29616: F, t29618: F, t29620: F) -> (F, F, F) {
    let t30356 = t2061 * t6071;
    let t30357 = t7071 * t30356;
    let t30378 = t26462 + t29623 / 8.0 - 0.10164000561857065645e-3 * t27228 + 0.80031500487063509014e-2 * t27230 + 0.17149607247227894789e-1 * t29627 - t29629 / 24.0 + 0.32012600194825403606e-1 * t27256 + t26468 - t26471 - 0.85748036236139473944e-3 * t29631 - 0.34299214494455789578e-2 * t29633;
    let t30379 = t26450 - t26454 + t26457 + 0.22866142996303859718e-3 * t27240 + 0.17149607247227894789e-2 * t29616 + 0.68598428988911579156e-2 * t29618 - 0.85748036236139473944e-3 * t29620 - 0.4065600224742826258e-3 * t27251 + 0.57165357490759649296e-4 * t27254 + 7.0 / 36.0 * t27246 + t30378;
    (t30356, t30357, t30379)
}
