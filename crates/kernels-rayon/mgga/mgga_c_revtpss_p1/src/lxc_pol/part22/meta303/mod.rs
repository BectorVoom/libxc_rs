//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta303(t10073: f64, t4089: f64, t1398: f64, t1419: f64, t4086: f64, t543: f64, t2782: f64, t4056: f64, t555: f64, t9990: f64, t1432: f64, t2470: f64, t4107: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10074, t10079, t10080, t10084, t10085, t10090, t10098) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1738(t10073, t4089, t1398, t1419, t4086, t543, t2782, t4056, t555, t9990, t1432, t2470, t4107);
    (t10074, t10079, t10080, t10084, t10085, t10090, t10098)
}
