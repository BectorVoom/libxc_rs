//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta932 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3056;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3057;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3058;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3059;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3060;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3061;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta932(t24232: f64, t606: f64, t1120: f64, t128: f64, t16737: f64, t5825: f64, t18281: f64, t5051: f64, t22671: f64, t3367: f64, t22688: f64, t43766: f64, t43860: f64, t43995: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t51957: f64, t56250: f64, t77513: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81194, t81196) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3056(t24232, t606, t1120, t128);
        let (t81198, t81200) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3057(t16737, t5825, t1120, t128);
        let (t81202, t81204) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3058(t18281, t5051, t1120, t128);
        let (t81207, t81209) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3059(t22671, t3367, t606, t1120, t128);
        let (t81212, t81214) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3060(t22688, t43766, t606, t128, t43860);
        let t81218 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3061(t43995, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t81224 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3062(t51957, t56250, t77513);
    (t81194, t81196, t81198, t81200, t81202, t81204, t81207, t81209, t81212, t81214, t81218, t81224)
}
