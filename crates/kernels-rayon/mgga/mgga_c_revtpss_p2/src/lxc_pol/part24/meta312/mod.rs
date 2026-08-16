//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta312(t6800: f64, t749: f64, t512: f64, t177: f64, t762: f64, t1877: f64, t73: f64, t4010: f64, t6836: f64, t1412: f64, t6816: f64, t221: f64, t4019: f64, t6844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22195, t22196, t22212, t22213, t22229, t22236, t22245, t22259) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1099(t6800, t749, t512, t177, t762, t1877, t73, t4010, t6836, t1412, t6816, t221, t4019, t6844);
    (t22195, t22196, t22212, t22213, t22229, t22236, t22245, t22259)
}
