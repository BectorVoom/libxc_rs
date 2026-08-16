//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1173/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1173(t3575: f64, t42386: f64, t11888: f64, t11914: f64, t11784: f64, t820: f64, t11779: f64, t11153: f64, t1176: f64, t11881: f64, t374: f64, t485: f64, t486: f64, t9697: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45113 = t3575 * t42386;
    let t45114 = t11888 * t45113;
    let t45119 = t11914 * t45113;
    let t45124 = t820 * t11784;
    let t45128 = t820 * t11779;
    let t45192 = t1176 * t11153;
    let t45197 = t11881 * t45113;
    let t45250 = 7.0_f64 / 31104.0_f64 * t485 * t374 * t9697 * t486;
    (t45114, t45119, t45124, t45128, t45192, t45197, t45250)
}
