//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta834 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2704;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2705;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta834(t20307: f64, t689: f64, t20349: f64, t698: f64, t20352: f64, t20343: f64, t20346: f64, t20273: f64, t2439: f64, t6467: f64, t6464: f64, t6461: f64, t20567: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68456 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2704(t20307, t689);
        let (t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590, t68609) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2705(t20349, t698, t20352, t20343, t20346, t20273, t2439, t6467, t6464, t6461, t20567, t300);
    (t68456, t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590, t68609)
}
