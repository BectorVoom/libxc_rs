//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3040/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3040(t78192: f64, t78195: f64, t78201: f64, t78203: f64, t78206: f64, t78246: f64, t78248: f64, t78251: f64, t78254: f64, t78303: f64, t78305: f64, t78307: f64, t78309: f64, t78311: f64, t78313: f64, t78315: f64, t78319: f64, t78322: f64, t78325: f64, t78328: f64, t78332: f64, t78335: f64) -> (f64, f64) {
    let t81076 = -t78192 - t78195 - t78201 + t78203 + t78206 + t78246 - t78248 - t78251 + t78254 - t78303 + t78305;
    let t81078 = -t78307 + t78309 - t78311 + t78313 + t78315 + t78319 - t78322 - t78325 - t78328 + t78332 + t78335;
    (t81076, t81078)
}
