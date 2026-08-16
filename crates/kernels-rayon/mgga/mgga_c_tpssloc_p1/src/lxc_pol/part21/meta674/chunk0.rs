//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2479/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2479(t111: f64, t12723: f64, t1454: f64, t2585: f64, t2281: f64, t4044: f64, t12758: f64, t626: f64, t12761: f64, t12754: f64, t4068: f64, t12809: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45632 = t12723 * t111;
    let t45656 = t2585 * t1454;
    let t45658 = t2281 * t4044;
    let t45660 = t626 * t12758;
    let t45662 = t626 * t12761;
    let t45676 = t626 * t12754;
    let t45688 = t2281 * t4068;
    let t45690 = t626 * t12809;
    (t45632, t45656, t45658, t45660, t45662, t45676, t45688, t45690)
}
