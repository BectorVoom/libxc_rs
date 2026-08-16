//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1265/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1265(t46104: f64, t38735: f64, t3854: f64, t2157: f64, t343: f64, t49847: f64, t46115: f64, t11419: f64, t44479: f64, t11414: f64, t37755: f64, t858: f64, t866: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t50115 = 7.0_f64 / 12.0_f64 * t46104;
    let t50116 = 35.0_f64 / 18.0_f64 * t38735;
    let t50117 = t3854 * t3854;
    let t50118 = t50117 * t2157;
    let t50123 = t49847 * t343;
    let t50128 = 7.0_f64 / 12.0_f64 * t46115;
    let t50135 = t11419 * t44479 / 4.0_f64;
    let t50137 = t37755 * t11414 / 4.0_f64;
    let t50142 = t50117 * t343;
    let t50146 = t866 * t867 * t858 * t50142 / 32.0_f64;
    (t50115, t50116, t50118, t50123, t50128, t50135, t50137, t50142, t50146)
}
