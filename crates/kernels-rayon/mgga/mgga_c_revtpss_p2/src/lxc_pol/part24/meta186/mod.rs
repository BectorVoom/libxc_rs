//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta186 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta186(t9484: f64, t9543: f64, t520: f64, t512: f64, t1333: f64, t3857: f64, t2626: f64, t676: f64, t3869: f64, t2434: f64, t762: f64, t186: f64, t685: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9544, t9545, t9546, t9569, t9572, t9574, t9575, t9577, t9586) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk907(t9484, t9543, t520, t512, t1333, t3857, t2626, t676, t3869, t2434, t762, t186, t685, t793);
    (t9544, t9545, t9546, t9569, t9572, t9574, t9575, t9577, t9586)
}
