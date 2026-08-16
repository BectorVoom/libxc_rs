//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta146 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk923;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk924;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta146(t1497: f64, t644: f64, t1469: f64, t606: f64, t30: f64, t33: f64, t70: f64, t2255: f64, zeta_threshold: f64, t36: f64, t1470: f64, t627: f64, t1486: f64, t607: f64, t2275: f64, t48: f64, t2282: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4178, t4181) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk923(t1497, t644, t1469, t606);
        let (t4182, t4186) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk924(t30, t33, t4181, t70, t2255, zeta_threshold);
        let (t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk925(t36, t4186, t70, t1470, t627, t1486, t607, t1469, t2275, t606, t48, t2282);
    (t4178, t4181, t4182, t4186, t4187, t4188, t4191, t4196, t4201, t4202, t4205, t4210)
}
