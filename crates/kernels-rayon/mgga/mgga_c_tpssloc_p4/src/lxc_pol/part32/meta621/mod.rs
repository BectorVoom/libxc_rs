//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2026;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta621(t27551: f64, t7327: f64, t135: f64, t24847: f64, t7284: f64, t1090: f64, t24821: f64, t1089: f64, t1235: f64, t11708: f64, t24728: f64, t11713: f64, t11715: f64, t11717: f64, sigma2: f64, t24649: f64, t24658: f64, t2131: f64, t82985: f64, t24727: f64, t24732: f64, t7337: f64, t11835: f64, t7310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86077, t86094, t86102, t86116, t86140, t86146) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2026(t27551, t7327, t135, t24847, t7284, t1090, t24821, t1089, t1235, t11708, t24728, t11713, t11715, t11717, sigma2);
        let (t86149, t86154, t86164, t86167, t86171, t86184) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2027(t24649, t24658, t2131, t82985, t11713, t11717, t24727, t11708, t24732, t7337, t11835, t7310);
    (t86077, t86094, t86102, t86116, t86140, t86146, t86149, t86154, t86164, t86167, t86171, t86184)
}
