//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1490/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1490(t2311: f64, t2319: f64, t107: f64, t9576: f64, t2585: f64, t667: f64, t2281: f64, t2333: f64, t2359: f64, t626: f64, t9367: f64, t9371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45408 = t2311 * t2319;
    let t45421 = 2618.0_f64 / 81.0_f64 * t9576 * t107;
    let t45422 = t2585 * t667;
    let t45424 = t2281 * t2333;
    let t45426 = t2281 * t2359;
    let t45428 = t626 * t9367;
    let t45430 = t626 * t9371;
    (t45408, t45421, t45422, t45424, t45426, t45428, t45430)
}
