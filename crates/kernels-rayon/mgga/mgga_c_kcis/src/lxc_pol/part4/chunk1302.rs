//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1302/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1302(t16693: f64, t16694: f64, t4170: f64, t16692: f64, t1307: f64, t6037: f64, t4162: f64, t4160: f64, t11862: f64, t5645: f64, t5650: f64, t5656: f64) -> (f64, f64, f64, f64, f64) {
    let t16695 = t16693 * t16694;
    let t16696 = t4170 * t16695;
    let t16697 = t16692 * t16696;
    let t16700 = t6037 * t1307;
    let t16701 = t4162 * t16700;
    let t16702 = t4160 * t16701;
    let t16704 = t11862 * t5645;
    let t16706 = t11862 * t5650;
    let t16708 = t11862 * t5656;
    (t16697, t16702, t16704, t16706, t16708)
}
