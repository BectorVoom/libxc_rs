//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta947 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3185;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta947(t12866: f64, t17514: f64, t56756: f64, t12916: f64, t17723: f64, t3718: f64, t12832: f64, t17617: f64, t12851: f64, t1778: f64, t17429: f64, t17789: f64, t12910: f64, t17624: f64, t17709: f64, t17712: f64, t3766: f64, t5219: f64, t5330: f64, t17601: f64, t12855: f64, t17579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59078, t59094, t59142, t59144, t59146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3185(t12866, t17514, t56756, t12916, t17723, t3718, t12832, t17617, t12851, t1778, t17429, t17789);
        let (t59149, t59159, t59162, t59173, t59176) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3186(t12910, t12916, t17624, t17709, t17712, t3766, t5219, t5330, t17601, t3718, t12855, t17579);
    (t59078, t59094, t59142, t59144, t59146, t59149, t59159, t59162, t59173, t59176)
}
