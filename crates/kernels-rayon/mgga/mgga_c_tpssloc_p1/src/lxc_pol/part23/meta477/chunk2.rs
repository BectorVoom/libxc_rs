//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1431/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1431(t423: f64, t78266: f64, t78278: f64, t21961: f64, t51249: f64, t11275: f64, t3315: f64, t78129: f64, t6068: f64) -> (f64, f64, f64, f64) {
    let t78281 = 0.621814e-1_f64 * (t78266 + t78278) * t423;
    let t78283 = 0.3859675079686208416e3_f64 * t51249 * t21961;
    let t78286 = 0.57895126195293126241e3_f64 * t11275 * t78129 * t3315;
    let t78287 = t6068 * t6068;
    (t78281, t78283, t78286, t78287)
}
