//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1802/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1802(t2061: f64, t6071: f64, t7071: f64, t26462: f64, t26468: f64, t26471: f64, t27228: f64, t27230: f64, t27256: f64, t29623: f64, t29627: f64, t29629: f64, t29631: f64, t29633: f64) -> (f64, f64, f64) {
    let t30356 = t2061 * t6071;
    let t30357 = t7071 * t30356;
    let t30378 = t26462 + t29623 / 8.0_f64 - 0.10164000561857065645e-3_f64 * t27228 + 0.80031500487063509014e-2_f64 * t27230 + 0.17149607247227894789e-1_f64 * t29627 - t29629 / 24.0_f64 + 0.32012600194825403606e-1_f64 * t27256 + t26468 - t26471 - 0.85748036236139473944e-3_f64 * t29631 - 0.34299214494455789578e-2_f64 * t29633;
    (t30356, t30357, t30378)
}
