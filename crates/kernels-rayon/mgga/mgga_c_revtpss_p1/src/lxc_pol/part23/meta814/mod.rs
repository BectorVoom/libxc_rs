//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta814 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta814(t11120: f64, t1651: f64, t1071: f64, t19462: f64, t19856: f64, t378: f64, t1647: f64, t4930: f64, t3056: f64, t6234: f64, t15669: f64, t379: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t64614, t64629, t64636, t64639, t64686, t64687, t64711) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2659(t11120, t1651, t1071, t19462, t19856, t378, t1647, t4930, t3056, t6234, t15669, t379);
    (t64614, t64629, t64636, t64639, t64686, t64687, t64711)
}
