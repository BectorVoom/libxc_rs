//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1111/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1111(t28624: f64, t6012: f64, t27544: f64, t5916: f64, t27543: f64, t576: f64, t5905: f64, t1528: f64, t2043: f64, t27514: f64, t8191: f64, t5919: f64, t7948: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28625 = t28624 * t6012;
    let t28627 = t27544 * t5916;
    let t28629 = t576 * t27543;
    let t28630 = t28629 * t5905;
    let t28632 = t1528 * t2043;
    let t28634 = t27514 * t8191;
    let t28636 = t7948 * t5919;
    (t28625, t28627, t28629, t28630, t28632, t28634, t28636)
}
