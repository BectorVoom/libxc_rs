//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta450(t10199: f64, t1514: f64, t4398: f64, t9372: f64, t9387: f64, t14362: f64, t9575: f64, t9318: f64, t10565: f64, t1469: f64, t706: f64, t1531: f64, t36: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t49698, t49866, t49897, t49926, t49940, t50084, t50089) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1414(t10199, t1514, t4398, t9372, t9387, t14362, t9575, t9318, t10565, t1469, t706, t1531, t36);
    (t49698, t49866, t49897, t49926, t49940, t50084, t50089)
}
