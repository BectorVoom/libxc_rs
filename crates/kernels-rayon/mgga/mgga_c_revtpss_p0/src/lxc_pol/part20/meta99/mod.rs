//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk565;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk566;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk567;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk568;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk569;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk570;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk571;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk572;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk573;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk574;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk575;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta99(t231: f64, t268: f64, t675: f64, t836: f64, t2798: f64, t72: f64, t860: f64, t686: f64, t874: f64, t2470: f64, t875: f64, t251: f64, t2718: f64, t822: f64, t213: f64, t234: f64, t2646: f64, t2724: f64, t2754: f64, t2760: f64, t2776: f64, t2780: f64, t2787: f64, t2791: f64, t2796: f64, t820: f64, t837: f64, t879: f64, t868: f64, t2437: f64, t2443: f64, t2446: f64, t2449: f64, t2460: f64, t2462: f64, t2468: f64, t2473: f64, t257: f64, t2761: f64, t2765: f64, t2772: f64, t865: f64, t887: f64, t198: f64, t207: f64, t2392: f64, t2393: f64, t2394: f64, t2400: f64, t2402: f64, t2403: f64, t2404: f64, t2408: f64, t2411: f64, t2416: f64, t2430: f64, t2569: f64, t2614: f64, t2617: f64, t765: f64, t775: f64, t892: f64, t2498: f64, t2518: f64, t2522: f64, t2525: f64, t2527: f64, t2562: f64, t2579: f64, t2587: f64, t2610: f64, t2621: f64, t2624: f64, t2628: f64, t2632: f64, t1941: f64, t271: f64, t689: f64, t907: f64, t1065: f64, t159: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2801, t2802, t2804, t2806, t2810, t2811) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk565(t231, t268, t675, t836, t2798, t72, t860, t686, t874, t2470, t875, t251, t2718);
        let t2815 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk566(t822, t860);
        let t2828 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk567(t213, t234, t2646, t2724, t2754, t2760, t2776, t2780, t2787, t2791, t2796, t2802, t2806, t2810, t2811, t2815, t820, t837, t879);
        let t2829 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk568(t2828, t868);
        let t2832 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk569(t213, t2437, t2443, t2446, t2449, t2460, t2462, t2468, t2473, t257, t2761, t2765, t2772, t2829, t865, t887);
        let t2836 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk570(t198, t207, t2392, t2393, t2394, t2400, t2402, t2403, t2404, t2408, t2411, t2416, t2430, t2569, t2614, t2617, t2832, t765, t775, t892);
        let t2837 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk571(t2498, t2518, t2522, t2525, t2527, t2562, t2579, t2587, t2610, t2621, t2624, t2628, t2632);
        let t2838 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk572(t2836, t2837);
        let t2846 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk573(t1941, t268, t271);
        let (t2847, t2848) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk574(t2846, t689, t907);
        let t2850 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk575(t1065, t159);
    (t2801, t2804, t2811, t2815, t2828, t2829, t2832, t2838, t2846, t2847, t2848, t2850)
}
