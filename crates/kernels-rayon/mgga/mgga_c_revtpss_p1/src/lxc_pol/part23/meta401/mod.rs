//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta401(t1260: f64, t5261: f64, t3647: f64, t5378: f64, t247: f64, t3634: f64, t5056: f64, t1261: f64, t12916: f64, t5334: f64, t5331: f64, t1778: f64, t3682: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17763, t17767, t17769, t17771, t17789, t17791, t17792) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1770(t1260, t5261, t3647, t5378, t247, t3634, t5056, t1261, t12916, t5334, t5331, t1778, t3682);
    (t17763, t17767, t17769, t17771, t17789, t17791, t17792)
}
