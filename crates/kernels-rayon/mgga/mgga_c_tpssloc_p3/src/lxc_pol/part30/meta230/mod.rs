//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta230 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1054;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta230(t1118: f64, t6020: f64, t1099: f64, t3315: f64, t5988: f64, t3313: f64, t3319: f64, t4721: f64, t5973: f64, t5977: f64, t5981: f64, t1682: f64, t1137: f64, t3339: f64, t3346: f64, t4770: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64, t6012: f64, t6015: f64, t6018: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6021, t6023, t6024, t6026, t6031, t6036) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1054(t1118, t6020, t1099, t3315, t5988, t3313, t3319, t4721, t5973, t5977, t5981, t1682);
        let (t6037, t6052) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1055(t1137, t6036, t3339, t3346, t4721, t4770, t5973, t5977, t5981, t5993, t6000, t6006, t6008, t6012, t6015, t6018);
    (t6021, t6023, t6024, t6026, t6031, t6036, t6037, t6052)
}
