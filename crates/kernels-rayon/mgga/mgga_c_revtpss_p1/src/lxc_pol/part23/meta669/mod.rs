//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta669 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta669(t11626: f64, t358: f64, t3145: f64, t3153: f64, t3154: f64, t1036: f64, t11240: f64, t42646: f64, t11255: f64, t42668: f64, t11245: f64, t2434: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42862, t42865, t42871, t42872, t42879, t42914, t42973, t42994) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2403(t11626, t358, t3145, t3153, t3154, t1036, t11240, t42646, t11255, t42668, t11245, t2434, t246);
    (t42862, t42865, t42871, t42872, t42879, t42914, t42973, t42994)
}
