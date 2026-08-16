//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1426;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1427;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta241(t3240: f64, t5971: f64, t123: f64, t3247: f64, t5392: f64, t1088: f64, t1089: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t5972, t5973, t5975) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1426(t3240, t5971, t123, t3247, t5392);
        let (t5976, t5977, t5979) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1427(t1088, t5975, t123, t1089, t5398);
    (t5972, t5973, t5975, t5976, t5977, t5979)
}
