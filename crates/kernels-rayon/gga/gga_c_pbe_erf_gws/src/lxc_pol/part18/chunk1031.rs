//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1031/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1031(t11514: f64, t254: f64, t2157: f64, t3222: f64, t1076: f64, t820: f64, t274: f64, t3258: f64, t3257: f64, t1105: f64, t816: f64, t1109: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11539 = t254 * t11514;
    let t11540 = t2157 * param_a_c;
    let t11541 = t11540 * t3222;
    let t11542 = t11539 * t11541;
    let t11545 = t1076 * t820;
    let t11546 = t11545 * t274;
    let t11547 = t3258 * t11546;
    let t11548 = t3257 * t11547;
    let t11551 = t816 * t1105;
    let t11552 = t11551 * t820;
    let t11553 = t3258 * t11552;
    let t11554 = t3257 * t11553;
    let t11557 = t816 * t1109;
    (t11539, t11541, t11542, t11547, t11548, t11553, t11554, t11557)
}
