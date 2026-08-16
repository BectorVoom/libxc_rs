//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta225(t207: f64, t242: f64, t240: f64, t72: f64, t10627: f64, t828: f64, t136: f64, t2476: f64, t221: f64, t2394: f64, t2674: f64, t231: f64, t243: f64, t2645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10696, t10697, t10698, t10700, t10703, t10705, t10706, t10709) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1015(t207, t242, t240, t72, t10627, t828, t136, t2476, t221, t2394, t2674, t231, t243, t2645);
    (t10696, t10697, t10698, t10700, t10703, t10705, t10706, t10709)
}
