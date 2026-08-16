//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1276/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1276(t1332: f64, t31175: f64, t8467: f64, t2690: f64, t544: f64, t553: f64, t1351: f64, t22705: f64, t22852: f64, t550: f64, t59: f64, t22751: f64, t31195: f64) -> (f64, f64, f64, f64) {
    let t114034 = t1332 * t31175 * t8467;
    let t114035 = 7.0_f64 / 1152.0_f64 * t114034;
    let t114038 = t544 * t553 * t2690 * t8467;
    let t114046 = t22852 * t22705 * t59 * t1351 * t550;
    let t114057 = t22751 * t31195;
    (t114035, t114038, t114046, t114057)
}
