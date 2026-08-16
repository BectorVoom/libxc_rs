//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1165/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1165(t2115: f64, t6616: f64, t2129: f64, t2087: f64, t2189: f64, t810: f64, t3140: f64, t3138: f64, t4386: f64, t2112: f64, t2079: f64, t2319: f64, t6466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20796 = t2115 * t6616;
    let t20797 = 35.0_f64 / 72.0_f64 * t20796;
    let t20798 = t2129 * t6616;
    let t20799 = 35.0_f64 / 72.0_f64 * t20798;
    let t20800 = t2087 * t6616;
    let t20801 = 35.0_f64 / 72.0_f64 * t20800;
    let t20802 = t810 * t2189;
    let t20803 = t3140 * t20802;
    let t20806 = t3138 * t4386 * t20803 / 2.0_f64;
    let t20807 = t2112 * t2112;
    let t20808 = t2079 * t20807;
    let t20813 = t2319 * t6466;
    (t20797, t20799, t20801, t20802, t20803, t20806, t20807, t20808, t20813)
}
