//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk594;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk595;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk596;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta104(t290: f64, t2875: f64, t2924: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t941: f64, t945: f64, t307: f64, t944: f64, t302: f64, t953: f64, t954: f64, t2904: f64, t2882: f64, t2890: f64, t2898: f64, t2900: f64, t2906: f64, t2910: f64, t2913: f64, t2916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2925, t2926) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk594(t290);
        let (t2927, t2929, t2935, t2938, t2941) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk595(t2875, t2926, t2924, t2846, t2848, t2855, t2860, t2864, t941, t945, t307, t944);
        let (t2942, t2943, t2944) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk596(t2941, t302, t953);
        let (t2945, t2962) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk597(t2944, t954, t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
    (t2925, t2926, t2927, t2929, t2935, t2938, t2941, t2942, t2943, t2944, t2945, t2962)
}
