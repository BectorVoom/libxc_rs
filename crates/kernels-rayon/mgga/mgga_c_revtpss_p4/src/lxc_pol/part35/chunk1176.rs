//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1176/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1176(t110322: f64, t25375: f64, t18805: f64, t95936: f64, t30391: f64, t689: f64, t93314: f64, t93302: f64, t30313: f64, t531: f64, t116: f64, t30570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110615 = t25375 * t110322;
    let t110639 = t95936 * t18805;
    let t110676 = t30391 * t689;
    let t110677 = t93314 * t110676;
    let t110679 = t93302 * t110676;
    let t111221 = t531 * t30313;
    let t111320 = t116 * t30570;
    (t110615, t110639, t110677, t110679, t111221, t111320)
}
