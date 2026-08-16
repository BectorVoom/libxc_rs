//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta786 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2597;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta786(t18281: f64, t189: f64, t18555: f64, t2619: f64, t14341: f64, t4311: f64, t18562: f64, t2516: f64, t2496: f64, t5825: f64, t749: f64, t4401: f64, t606: f64, t14369: f64, t4186: f64, t2439: f64, t6041: f64, t780: f64, t785: f64, t18821: f64, t2471: f64, t18814: f64, t2435: f64, t14476: f64, t1580: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61266, t61282, t61289, t61294, t61296, t61303, t61305) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2597(t18281, t189, t18555, t2619, t14341, t4311, t18562, t2516, t2496, t5825, t749, t4401, t606);
        let (t61315, t61324, t61330, t61337, t61344) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2598(t14369, t4186, t4401, t2439, t6041, t780, t785, t18821, t2471, t18814, t2435, t14476, t1580, t689);
    (t61266, t61282, t61289, t61294, t61296, t61303, t61305, t61315, t61324, t61330, t61337, t61344)
}
