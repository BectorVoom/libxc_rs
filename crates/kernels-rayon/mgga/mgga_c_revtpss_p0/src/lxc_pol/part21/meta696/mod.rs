//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta696 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta696(t12627: f64, t3754: f64, t1209: f64, t17887: f64, t12657: f64, t12722: f64, t3555: f64, t12640: f64, t3552: f64, t3766: f64, t5462: f64, t5477: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45666, t45683, t45697, t45700, t45707, t45710, t45715, t45718) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2518(t12627, t3754, t1209, t17887, t12657, t12722, t3555, t12640, t3552, t3766, t5462, t5477);
    (t45666, t45683, t45697, t45700, t45707, t45710, t45715, t45718)
}
