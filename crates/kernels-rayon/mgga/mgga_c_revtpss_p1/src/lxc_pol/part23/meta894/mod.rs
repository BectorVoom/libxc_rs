//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta894 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2851;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta894(t61130: f64, t10439: f64, t22688: f64, t750: f64, t49926: f64, t18263: f64, t4308: f64, t49940: f64, t23211: f64, t72: f64, t757: f64, t61165: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39764: f64, t39770: f64, t39773: f64, t49930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76963, t76966, t76967, t76969, t76970, t76973, t76974) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2851(t61130, t10439, t22688, t750, t49926, t18263, t4308, t49940, t23211, t72, t757, t61165);
        let t76975 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2852(t39741, t39744, t39747, t39750, t39756, t39760, t39764, t39770, t39773, t49930, t76963, t76966, t76967, t76969, t76970, t76973, t76974);
    (t76963, t76966, t76967, t76969, t76970, t76973, t76974, t76975)
}
