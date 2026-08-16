//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2012/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2012(t94483: f64, t64: f64, t9990: f64, t2482: f64, t596: f64, t7262: f64, t4021: f64, t25981: f64, t27: f64, t550: f64, t7021: f64, t25273: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94484 = 0.91476005056713590805e-4_f64 * t94483;
    let t94491 = t9990 * t64;
    let t94497 = t2482 * t7262 * t596;
    let t94498 = t94497 * t4021;
    let t94508 = t2482 * t25981 * t27;
    let t94513 = t7021 * t550;
    let t94519 = t25273 * t540;
    (t94484, t94491, t94497, t94498, t94508, t94513, t94519)
}
