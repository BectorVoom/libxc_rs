//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1246;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1247;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta330(t1042: f64, t13080: f64, t247: f64, t3372: f64, t3634: f64, t1261: f64, t3368: f64, t3636: f64, t3647: f64, t12646: f64, t3719: f64, t3367: f64, t414: f64, t66: f64, t12257: f64, t1222: f64, t1247: f64, t1252: f64, t13008: f64, t13012: f64, t13015: f64, t13018: f64, t13022: f64, t13029: f64, t13033: f64, t13042: f64, t13048: f64, t13052: f64, t13055: f64, t13058: f64, t13062: f64, t13065: f64, t13069: f64, t13076: f64, t3591: f64, t3606: f64, t3613: f64, t3708: f64, t5384: f64, t12845: f64, t12929: f64, t13005: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13081, t13085, t13086, t13089, t13090, t13092, t13095, t13099) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1246(t1042, t13080, t247, t3372, t3634, t1261, t3368, t3636, t3647, t12646, t3719, t3367, t414);
        let (t13100, t13102, t13105) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1247(t13099, t66, t12257, t247, t1222, t1247, t1252, t1261, t13008, t13012, t13015, t13018, t13022, t13029, t13033, t13042, t13048, t13052, t13055, t13058, t13062, t13065, t13069, t13076, t13081, t13086, t13090, t13092, t13095, t3591, t3606, t3613, t3708, t5384);
        let t13107 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1248(t12845, t12929, t13005, t13105);
    (t13081, t13085, t13089, t13095, t13099, t13100, t13102, t13107)
}
