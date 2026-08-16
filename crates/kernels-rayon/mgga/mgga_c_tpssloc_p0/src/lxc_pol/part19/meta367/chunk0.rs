//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1344/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1344(t41961: f64, t41845: f64, t41863: f64, t41865: f64, t41868: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t41882: f64, t41885: f64, t41973: f64) -> f64 {
    let t43002 = 220.0_f64 / 81.0_f64 * t41961;
    let t43012 = -t41845 - 4.0_f64 / 3.0_f64 * t41973 - t43002 - 160.0_f64 / 81.0_f64 * t41863 + 8.0_f64 / 9.0_f64 * t41865 - 8.0_f64 / 9.0_f64 * t41868 + 10.0_f64 / 9.0_f64 * t41870 + 10.0_f64 / 27.0_f64 * t41872 - 4.0_f64 / 9.0_f64 * t41874 - 16.0_f64 / 81.0_f64 * t41876 + 14.0_f64 / 81.0_f64 * t41882 + t41885 / 6.0_f64;
    t43012
}
