//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2387;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2388;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta559(t3568: f64, t5486: f64, t1287: f64, t1794: f64, t3727: f64, t1770: f64, t3766: f64, t3759: f64, t5245: f64, t5457: f64, t5351: f64, t13126: f64, t487: f64, t460: f64, t12050: f64, t3601: f64, t471: f64, t17710: f64, t1204: f64, t5462: f64, t3754: f64, t5219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17917, t17921, t17934, t17941, t17944, t17945, t17948) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2387(t3568, t5486, t1287, t1794, t3727, t1770, t3766, t3759, t5245, t5457, t5351, t13126, t487);
        let t17949 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2388(t17948, t460);
        let (t17951, t17952, t17955, t17958) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2389(t12050, t3601, t471, t17710, t1204, t5462, t3754, t5219);
    (t17917, t17921, t17934, t17941, t17944, t17945, t17948, t17949, t17951, t17952, t17955, t17958)
}
