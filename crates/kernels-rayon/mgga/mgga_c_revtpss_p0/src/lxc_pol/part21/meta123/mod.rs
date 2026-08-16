//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk791;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk792;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk793;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk794;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta123(t290: f64, t2875: f64, t2924: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t941: f64, t945: f64, t307: f64, t944: f64, t302: f64, t953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2925, t2926) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk791(t290);
        let (t2927, t2929, t2930, t2935, t2938) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk792(t2875, t2926, t2924, t2846, t2848, t2855, t2860, t2864, t941, t945);
        let (t2941, t2942) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk793(t307, t944);
        let t2943 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk794(t2942, t302);
        let t2944 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk795(t953);
    (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2941, t2942, t2943, t2944)
}
