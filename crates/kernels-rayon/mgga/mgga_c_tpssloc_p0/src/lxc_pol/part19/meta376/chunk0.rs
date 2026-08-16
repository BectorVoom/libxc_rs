//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1401/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1401(t43748: f64, t43750: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43794: f64, t43798: f64, t43802: f64, t43806: f64, t11778: f64, t154: f64) -> (f64, f64) {
    let t43808 = -16.0_f64 / 27.0_f64 * t43748 - 40.0_f64 / 81.0_f64 * t43750 + 8.0_f64 / 9.0_f64 * t43780 + 16.0_f64 / 9.0_f64 * t43782 + 16.0_f64 / 9.0_f64 * t43784 - 8.0_f64 / 3.0_f64 * t43786 - 4.0_f64 / 9.0_f64 * t43788 + 40.0_f64 / 9.0_f64 * t43794 - 8.0_f64 * t43798 + 8.0_f64 * t43802 + t43806 / 3.0_f64;
    let t43809 = t154 * t11778;
    (t43808, t43809)
}
