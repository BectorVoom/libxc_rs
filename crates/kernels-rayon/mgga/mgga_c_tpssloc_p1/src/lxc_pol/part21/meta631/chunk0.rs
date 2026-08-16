//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2414/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2414(t2649: f64, t41115: f64, t2553: f64, t2632: f64, t10024: f64, t809: f64, t2614: f64, t2693: f64, t238: f64, t244: f64, t248: f64, t40445: f64) -> (f64, f64, f64, f64, f64) {
    let t41116 = t41115 * t2649;
    let t41123 = t2632 * t2553;
    let t41130 = t809 * t10024;
    let t41134 = t2614 * t2693;
    let t41139 = 13685.0_f64 / 31104.0_f64 * t238 * t40445 * t244 * t248;
    (t41116, t41123, t41130, t41134, t41139)
}
