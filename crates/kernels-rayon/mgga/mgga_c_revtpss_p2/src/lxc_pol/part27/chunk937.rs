//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 937/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk937(t11249: f64, t3154: f64, t11248: f64, t1042: f64, t1036: f64, t11244: f64, t11240: f64, t357: f64, t246: f64, t676: f64, t1046: f64, t1041: f64) -> (f64, f64, f64, f64, f64) {
    let t11250 = t11249 * t3154;
    let t11251 = t11248 * t11250;
    let t11252 = t1042 * t11251;
    let t11255 = t1036 * t11244;
    let t11256 = t11240 * t11255;
    let t11257 = t11249 * t357;
    let t11258 = t11248 * t11257;
    let t11259 = t1042 * t11258;
    let t11262 = t246 * t676;
    let t11263 = t11262 * t1046;
    let t11264 = t1041 * t11263;
    (t11252, t11256, t11259, t11262, t11264)
}
