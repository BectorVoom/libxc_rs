//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta117 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk791;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk792;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk793;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk794;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk795;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk796;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk797;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk798;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk799;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk800;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta117(t2498: f64, t2518: f64, t2522: f64, t2525: f64, t2527: f64, t2562: f64, t2579: f64, t2587: f64, t2610: f64, t2621: f64, t2624: f64, t2628: f64, t2632: f64, t2836: f64, t1941: f64, t268: f64, t271: f64, t689: f64, t907: f64, t1065: f64, t159: f64, t631: f64, t2251: f64, t128: f64, t2297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2837 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk791(t2498, t2518, t2522, t2525, t2527, t2562, t2579, t2587, t2610, t2621, t2624, t2628, t2632);
        let t2838 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk792(t2836, t2837);
        let t2846 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk793(t1941, t268, t271);
        let (t2847, t2848) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk794(t2846, t689, t907);
        let t2850 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk795(t1065, t159);
        let t2851 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk796(t631);
        let t2852 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk797(t2851);
        let t2853 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk798(t2251, t2852);
        let (t2854, t2855) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk799(t2850, t2853, t128);
        let t2857 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk800(t2297);
        let t2858 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk801(t2251, t2857);
    (t2838, t2846, t2847, t2848, t2850, t2851, t2852, t2853, t2854, t2855, t2857, t2858)
}
