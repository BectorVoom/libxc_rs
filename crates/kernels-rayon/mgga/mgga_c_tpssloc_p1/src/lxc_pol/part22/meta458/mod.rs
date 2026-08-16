//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta458(t4049: f64, t5396: f64, t20215: f64, t95: f64, t5415: f64, t1449: f64, t5480: f64, t9398: f64, t4059: f64, t5484: f64, t103: f64, t100: f64, t104: f64, t1447: f64, t1450: f64, t20312: f64, t5475: f64, t5481: f64, t5485: f64, t92: f64, tau1: f64, t109: f64, t656: f64, t12747: f64, t19471: f64, t19480: f64, t20305: f64, t20308: f64, t64: f64, t9358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20315, t20318, t20319, t20322, t20331, t20338, t20342) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1829(t4049, t5396, t20215, t95, t5415, t1449, t5480, t9398, t4059, t5484, t103, t100, t104, t1447, t1450, t20312, t5475, t5481, t5485, t92, tau1);
        let (t20343, t20347) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1830(t109, t20342, t656, t12747, t19471, t19480, t20305, t20308, t64, t9358);
    (t20315, t20318, t20319, t20322, t20331, t20338, t20342, t20343, t20347)
}
