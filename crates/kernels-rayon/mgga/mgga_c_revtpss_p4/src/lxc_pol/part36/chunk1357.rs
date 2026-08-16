//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1357/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1357(t104721: f64, t104927: f64, t104988: f64, t104990: f64, t104999: f64, t112373: f64, t112480: f64, t1785: f64, t1791: f64, t2137: f64, t2138: f64, t24244: f64, t24679: f64, t24699: f64, t24787: f64, t26867: f64, t29047: f64, t29048: f64, t29086: f64, t30815: f64, t467: f64, t484: f64, t6601: f64, t6611: f64, t6640: f64, t6647: f64, t8184: f64) -> f64 {
    let t116290 = 0.15244095330869239812e-2_f64 * t104988 + t104990 / 432.0_f64 - 0.10620053080505570402e0_f64 * t467 * t2137 * t24679 * t484 + 0.42874018118069736972e-3_f64 * t24699 * t2138 * t484 + 0.43445671692977333464e-1_f64 * t1785 * t30815 * t484 - 0.68598428988911579154e-2_f64 * t6601 * t8184 * t484 - 0.28582678745379824648e-3_f64 * t104999 + 0.91464571985215438873e-2_f64 * t104721 * t6640 - 0.85748036236139473944e-3_f64 * t26867 * t24787 - 0.12862205435420921092e-2_f64 * t29086 * t6647 + 0.25724410870841842183e-2_f64 * t104927 * t6611 - 0.12862205435420921092e-2_f64 * t112373 * t1791 + 0.13719685797782315831e-1_f64 * t112480 * t1791 - t29047 * t29048 * t24244 / 48.0_f64;
    t116290
}
