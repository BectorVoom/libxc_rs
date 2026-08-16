//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta599(t676: f64, t9387: f64, t2629: f64, t9372: f64, t2434: f64, t2516: f64, t8779: f64, t9645: f64, t252: f64, t685: f64, t788: f64, t10115: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39532, t39534, t39535, t39537, t39538, t39540, t39545, t39549, t39550) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2324(t676, t9387, t2629, t9372, t2434, t2516, t8779, t9645, t252, t685, t788, t10115, t862);
    (t39532, t39534, t39535, t39537, t39538, t39540, t39545, t39549, t39550)
}
