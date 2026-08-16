//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2276/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2276(t1458: f64, t2311: f64, t1873: f64, t22479: f64, t7676: f64, t7467: f64, t9348: f64, t45632: f64, t111: f64, t26097: f64, t12734: f64, t2314: f64, t26135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t90381 = t2311 * t1458;
    let t90383 = 2.0_f64 * t90381 * t1873;
    let t90385 = 2.0_f64 * t7676 * t22479;
    let t90387 = 2.0_f64 * t9348 * t7467;
    let t90399 = 2.0_f64 * t45632 * t1873;
    let t90400 = t26097 * t111;
    let t90404 = 4.0_f64 * t12734 * t7467;
    let t90406 = 4.0_f64 * t2314 * t26135;
    (t90381, t90383, t90385, t90387, t90399, t90400, t90404, t90406)
}
