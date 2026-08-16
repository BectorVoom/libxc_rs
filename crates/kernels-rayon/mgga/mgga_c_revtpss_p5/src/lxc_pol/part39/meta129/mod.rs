//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk633;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta129(t290: f64, t2875: f64, t2924: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t941: f64, t945: f64, t307: f64, t944: f64, t302: f64, t953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2941) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk633(t290, t2875, t2924, t2846, t2848, t2855, t2860, t2864, t941, t945, t307, t944);
        let (t2942, t2943, t2944) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk634(t2941, t302, t953);
    (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2942, t2943, t2944)
}
