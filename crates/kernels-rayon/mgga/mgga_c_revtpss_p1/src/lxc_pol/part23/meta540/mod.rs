//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2087;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta540(t221: f64, t6836: f64, t9921: f64, t3978: f64, t125: f64, t6816: f64, t1399: f64, t3936: f64, t6843: f64, t3938: f64, t5673: f64, t21990: f64, t5674: f64, t13944: f64, t6869: f64, t543: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22068, t22069, t22074, t22076, t22079) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2087(t221, t6836, t9921, t3978, t125, t6816, t1399, t3936, t6843);
        let (t22081, t22085, t22089, t22093, t22096) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2088(t22079, t3936, t3938, t1399, t5673, t21990, t5674, t13944, t6869, t543, t5591);
    (t22068, t22069, t22074, t22076, t22079, t22081, t22085, t22089, t22093, t22096)
}
