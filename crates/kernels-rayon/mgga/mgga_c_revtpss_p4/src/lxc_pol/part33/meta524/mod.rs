//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta524(t25904: f64, t27989: f64, t25899: f64, t2022: f64, t5774: f64, t7296: f64, t1955: f64, t5710: f64, t27960: f64, t545: f64, t2028: f64, t1904: f64, t2027: f64, t2030: f64, t26062: f64, t26065: f64, t26067: f64, t26071: f64, t26073: f64, t26084: f64, t27987: f64, t5728: f64, t7279: f64, t7292: f64, t7295: f64, t7308: f64, t7917: f64, t7930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27990, t27992, t28002, t28003, t28008, t28011, t28012, t28017) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1873(t25904, t27989, t25899, t2022, t5774, t7296, t1955, t5710, t27960, t545, t2028, t1904, t2027, t2030, t26062, t26065, t26067, t26071, t26073, t26084, t27987, t5728, t7279, t7292, t7295, t7308, t7917, t7930);
    (t27990, t27992, t28002, t28003, t28008, t28011, t28012, t28017)
}
