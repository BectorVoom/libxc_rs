//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk734;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk735;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk736;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk737;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk738;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta113(t2846: f64, t941: f64, t945: f64, t307: f64, t944: f64, t302: f64, t2904: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2930, t2938, t2941, t2942) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk734(t2846, t941, t945, t307, t944);
        let t2943 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk735(t2942, t302);
        let (t2950, t2957, t2966) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk736(t2846, t2904, t944);
        let t2967 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk737(t2966);
        let t2968 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk738(t2967, t302);
        let (t2969, t2970) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk739(t310);
    (t2930, t2938, t2941, t2942, t2943, t2950, t2957, t2966, t2967, t2968, t2969, t2970)
}
