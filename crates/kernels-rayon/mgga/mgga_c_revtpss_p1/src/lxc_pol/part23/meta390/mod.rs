//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1738;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta390(t17240: f64, t5052: f64, t1222: f64, t3636: f64, t5391: f64, t5381: f64, t1260: f64, t12966: f64, t1803: f64, t3666: f64, t1208: f64, t5215: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17241, t17243, t17258, t17260, t17261, t17283, t17288) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1738(t17240, t5052, t1222, t3636, t5391, t5381, t1260, t12966, t1803, t3666, t1208, t5215);
        let t17289 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1739(t17288, t225);
    (t17241, t17243, t17258, t17260, t17261, t17283, t17288, t17289)
}
