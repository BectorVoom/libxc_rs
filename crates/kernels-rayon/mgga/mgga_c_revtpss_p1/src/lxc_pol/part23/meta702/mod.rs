//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta702 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta702(t9860: f64, t9866: f64, t9863: f64, t3869: f64, t39532: f64, t9575: f64, t39538: f64, t39427: f64, t39535: f64, t4038: f64, t9372: f64, t1317: f64, t9428: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47125, t47127, t47131, t47135, t47138, t47140, t47142, t47147, t47149) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2452(t9860, t9866, t9863, t3869, t39532, t9575, t39538, t39427, t39535, t4038, t9372, t1317, t9428);
    (t47125, t47127, t47131, t47135, t47138, t47140, t47142, t47147, t47149)
}
