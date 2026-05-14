//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 964/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk964<F: Float>(t25283: F, t25251: F, t25256: F, t25258: F, t25263: F, t25267: F, t25271: F, t25278: F, t25280: F, t26462: F, t26468: F, t25223: F, t25225: F, t25229: F, t25235: F, t25238: F, t25246: F, t25248: F, t26450: F, t26454: F, t26457: F) -> (F,) {
    let t26471 = 0.10164000561857065645e-4 * t25283;
    let t26472 = -0.85748036236139473944e-3 * t25251 + t26462 + 0.22866142996303859718e-3 * t25256 - 0.85748036236139473944e-3 * t25258 + 0.17149607247227894789e-2 * t25263 + 0.80031500487063509014e-2 * t25267 + 0.68598428988911579156e-2 * t25271 + t26468 + 7.0 / 36.0 * t25278 - t25280 / 24.0 - t26471;
    let t26473 = t26450 + 0.32012600194825403606e-1 * t25223 - 0.34299214494455789578e-2 * t25225 + 0.57165357490759649296e-4 * t25229 - t26454 - 0.4065600224742826258e-3 * t25235 + t25238 / 8.0 + t26457 - 0.10164000561857065645e-3 * t25246 + 0.17149607247227894789e-1 * t25248 + t26472;
    (t26473,)
}
