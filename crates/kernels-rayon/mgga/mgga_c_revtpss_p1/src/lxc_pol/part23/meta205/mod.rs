//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta205 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1220;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1221;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1222;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta205(t5819: f64, t70: f64, t17: f64, t2255: f64, t30: f64, t33: f64, zeta_threshold: f64, t36: f64, t1470: f64, t1486: f64, t2275: f64, t48: f64, t476: f64, t53: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5820, t5823) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1220(t5819, t70, t17, t2255);
        let t5824 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1221(t5823);
        let t5825 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1222(t30, t33, t5824, zeta_threshold);
        let (t5826, t5827, t5830, t5835, t5838, t5842, t5843) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1223(t36, t5825, t70, t1470, t1486, t2275, t5819, t48, t476, t53, sigma2);
    (t5820, t5823, t5824, t5825, t5826, t5827, t5830, t5835, t5838, t5842, t5843)
}
