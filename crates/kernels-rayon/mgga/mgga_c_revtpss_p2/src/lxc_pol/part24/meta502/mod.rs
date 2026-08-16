//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1507;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta502(t10726: f64, t14586: f64, t18408: f64, t2661: f64, t23334: f64, t61625: f64, t10850: f64, t221: f64, t23172: f64, t2485: f64, t23281: f64, t2652: f64, t10858: f64, t23257: f64, t23279: f64, t10703: f64, t2674: f64, t2662: f64, t6035: f64, t61579: f64, t1559: f64, t18608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76583, t76587, t76591, t76593) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1507(t10726, t14586, t18408, t2661, t23334, t61625, t10850, t221, t23172, t2485, t23281, t2652);
        let (t76596, t76615, t76619, t76645) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1508(t10858, t23257, t221, t23279, t10703, t2674, t2661, t2662, t6035, t61579, t1559, t18608);
    (t76583, t76587, t76591, t76593, t76596, t76615, t76619, t76645)
}
