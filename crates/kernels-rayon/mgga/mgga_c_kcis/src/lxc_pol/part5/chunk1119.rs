//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1119/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1119(t2933: f64, t6390: f64, t6380: f64, t659: f64, t18681: f64, t945: f64, t26: f64, t6320: f64, t9752: f64, t934: f64, t4625: f64, t4700: f64) -> (f64, f64, f64, f64, f64) {
    let t18874 = 1.0_f64 * t2933 * t6390;
    let t18877 = t659 * t6380;
    let t18879 = t945 * t18681;
    let t18880 = t26 * t18879;
    let t18884 = t9752 * t6320;
    let t18885 = t18884 * t934;
    let t18887 = t4700 * t4625;
    (t18874, t18877, t18880, t18885, t18887)
}
