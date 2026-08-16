//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2204/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2204(t1454: f64, t2585: f64, t2281: f64, t4044: f64, t12758: f64, t626: f64, t12761: f64, t12754: f64, t4068: f64, t12809: f64, t92: f64, t9384: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45656 = t2585 * t1454;
    let t45658 = t2281 * t4044;
    let t45659 = 22.0_f64 / 3.0_f64 * t45658;
    let t45660 = t626 * t12758;
    let t45662 = t626 * t12761;
    let t45676 = t626 * t12754;
    let t45688 = t2281 * t4068;
    let t45689 = 11.0_f64 / 3.0_f64 * t45688;
    let t45690 = t626 * t12809;
    let t45697 = t92 * t9384;
    (t45656, t45659, t45660, t45662, t45676, t45689, t45690, t45697)
}
