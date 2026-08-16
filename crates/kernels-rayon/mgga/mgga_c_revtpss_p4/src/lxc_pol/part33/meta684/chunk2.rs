//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2253/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2253(t20783: f64, t26880: f64, t5326: f64, t8184: f64, t20846: f64, t26824: f64, t29062: f64, t5362: f64, t1256: f64, t30816: f64, t104972: f64, t112404: f64, t1238: f64, t20318: f64, t26827: f64, t29047: f64, t29048: f64, t29049: f64, t29083: f64, t5304: f64, t6647: f64, t97288: f64, t97296: f64) -> f64 {
    let t112468 = t26880 * t20783;
    let t112480 = t5326 * t8184;
    let t112483 = t26824 * t20846;
    let t112485 = t29062 * t5362;
    let t112487 = t30816 * t1256;
    let t112489 = 0.38110238327173099531e-3_f64 * t112468 - 0.5081365110289746604e-2_f64 * t29083 * t5304 - t104972 + 0.95275595817932748827e-4_f64 * t97288 + t97296 + t112404 * t29049 / 27.0_f64 - t29047 * t29048 * t20318 / 144.0_f64 - 0.42874018118069736972e-3_f64 * t26827 * t6647 + 0.45732285992607719436e-2_f64 * t112480 * t1238 + 0.57165357490759649296e-3_f64 * t112483 + 0.30488190661738479624e-2_f64 * t112485 + 0.96545937095505185473e-2_f64 * t112487;
    t112489
}
