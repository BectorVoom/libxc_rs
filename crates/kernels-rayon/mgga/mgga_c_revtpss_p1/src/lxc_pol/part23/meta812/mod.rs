//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta812 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta812(t19247: f64, t945: f64, t2967: f64, t6152: f64, t19021: f64, t3014: f64, t19045: f64, t964: f64, t3011: f64, t6184: f64, t2942: f64, t2923: f64, t6104: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t64055, t64060, t64072, t64120, t64125, t64319, t64336) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2657(t19247, t945, t2967, t6152, t19021, t3014, t19045, t964, t3011, t6184, t2942, t2923, t6104);
    (t64055, t64060, t64072, t64120, t64125, t64319, t64336)
}
