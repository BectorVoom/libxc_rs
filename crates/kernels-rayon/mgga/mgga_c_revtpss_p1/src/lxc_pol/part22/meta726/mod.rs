//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta726 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta726(t40169: f64, t760: f64, t2523: f64, t9372: f64, t39909: f64, t738: f64, t745: f64, t2251: f64, t2609: f64, t2611: f64, t36: f64, t716: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t40171, t40172, t40182, t40184, t40186, t40188) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2782(t40169, t760, t2523, t9372, t39909, t738, t745, t2251, t2609, t2611, t36, t716);
    (t40171, t40172, t40182, t40184, t40186, t40188)
}
