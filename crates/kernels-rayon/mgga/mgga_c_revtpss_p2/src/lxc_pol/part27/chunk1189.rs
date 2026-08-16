//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1189/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1189(t10310: f64, t77: f64, t84: f64, t2248: f64, t640: f64, t10298: f64, t607: f64, t2242: f64, t2259: f64, t25856: f64, t4254: f64, t13207: f64, t1936: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92692 = t77 * t84 * t10310;
    let t92696 = t77 * t640 * t2248;
    let t92709 = t10298 * t607;
    let t92711 = t2242 * t2259;
    let t92724 = 6.0_f64 * t4254 * t25856;
    let t92727 = 2.0_f64 * t651 * t13207 * t1936;
    (t92692, t92696, t92709, t92711, t92724, t92727)
}
