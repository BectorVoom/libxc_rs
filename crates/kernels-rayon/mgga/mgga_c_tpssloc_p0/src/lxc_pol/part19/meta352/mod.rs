//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1278;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1279;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta352(t909: f64, t9709: f64, t10310: f64, t699: f64, t10304: f64, t136: f64, t41688: f64, t2403: f64, t2833: f64, t2827: f64, t10322: f64, t10306: f64, t41678: f64, t41682: f64, t41684: f64, t41690: f64, t41699: f64, t41703: f64, t41711: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41863, t41865, t41868, t41870, t41872, t41874, t41876) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1278(t909, t9709, t10310, t699, t10304, t136, t41688, t2403, t2833, t2827, t10322, t10306);
        let t41878 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1279(t41678, t41682, t41684, t41690, t41699, t41703, t41711, t41863, t41865, t41868, t41870, t41872, t41874, t41876);
    (t41863, t41865, t41868, t41870, t41872, t41874, t41876, t41878)
}
