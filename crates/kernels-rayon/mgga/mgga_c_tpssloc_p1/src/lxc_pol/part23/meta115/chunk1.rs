//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 597/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk597(t1043: f64, t2775: f64, t2770: f64, t3061: f64, t135: f64, t1606: f64, t973: f64, t1036: f64, t1612: f64, t1616: f64, t248: f64, t3101: f64) -> (f64, f64, f64, f64, f64) {
    let t4583 = t1043 * t2775;
    let t4588 = t3061 * t2770;
    let t4603 = t135 * t1606;
    let t4604 = t973 * t4603;
    let t4625 = t1612 * t1036;
    let t4630 = t248 * t3101 * t1616;
    (t4583, t4588, t4604, t4625, t4630)
}
