//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1112;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1113;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta321(t22671: f64, t36: f64, t70: f64, t1486: f64, t5826: f64, t1470: f64, t5854: f64, t1469: f64, t5819: f64, t10355: f64, t4201: f64, t5825: f64, t48: f64, t477: f64, t53: f64, t10368: f64, t4210: f64, t60: f64, t10379: f64, t1480: f64, t1483: f64, t44: f64, t56: f64, t5843: f64, t5848: f64, t5851: f64, t61: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22672, t22673, t22676, t22681, t22688) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1112(t22671, t36, t70, t1486, t5826, t1470, t5854, t1469, t5819);
        let (t22699, t22700, t22709, t22712, t22715, t22718) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1113(t10355, t22688, t4201, t5825, t22671, t48, t477, t53, t10368, t4210, t60, t10379, t1480, t1483, t44, t56, t5843, t5848, t5851, t61, sigma2);
    (t22672, t22673, t22676, t22681, t22688, t22699, t22700, t22709, t22712, t22715, t22718)
}
