//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 710/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk710(t1065: f64, t6705: f64, t6704: f64, t1945: f64, t990: f64, t131: f64, t6679: f64, t1926: f64, t995: f64, t1919: f64, t210: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6706 = t6705 * t1065;
    let t6707 = t6704 * t6706;
    let t6710 = t990 * t1945;
    let t6712 = t6679 * t131;
    let t6716 = t1926 * t995 / 288.0_f64;
    let t6717 = t1919 * t210;
    (t6706, t6707, t6710, t6712, t6716, t6717)
}
