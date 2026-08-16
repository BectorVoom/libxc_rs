//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1498;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta274(t10008: f64, t225: f64, t1419: f64, t4086: f64, t786: f64, t4104: f64, t268: f64, t4056: f64, t543: f64, t675: f64, t4101: f64, t555: f64, t5744: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10009, t10013, t10014) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1498(t10008, t225, t1419, t4086, t786);
        let (t10015, t10019, t10020, t10022) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1499(t10014, t4104, t268, t4056, t543, t675, t4101, t555, t5744);
    (t10009, t10013, t10014, t10015, t10019, t10020, t10022)
}
