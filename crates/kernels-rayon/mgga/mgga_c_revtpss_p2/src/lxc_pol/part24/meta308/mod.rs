//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1093;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta308(t22022: f64, t2661: f64, t550: f64, t6861: f64, t4003: f64, t9934: f64, t3989: f64, t6856: f64, t3957: f64, t6884: f64, t6850: f64, t9744: f64, t125: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22023, t22025, t22026, t22027, t22028, t22030, t22038, t22044) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1093(t22022, t2661, t550, t6861, t4003, t9934, t3989, t6856, t3957, t6884, t6850, t9744);
        let t22046 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1094(t125, t6861);
    (t22023, t22025, t22026, t22027, t22028, t22030, t22038, t22044, t22046)
}
