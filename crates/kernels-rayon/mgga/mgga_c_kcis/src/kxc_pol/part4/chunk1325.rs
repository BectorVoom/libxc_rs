//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1325/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1325(t16685: f64, t5653: f64, t4170: f64, t4160: f64, t11425: f64, t556: f64, t16694: f64, t5661: f64, t1404: f64, t4035: f64, t1961: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t17005 = t5653 * t16685;
    let t17006 = t4170 * t17005;
    let t17007 = t4160 * t17006;
    let t17009 = t556 * t11425;
    let t17010 = t17009 * t16694;
    let t17011 = t4170 * t17010;
    let t17012 = t5661 * t17011;
    let t17019 = t1404 * t4035;
    let t17020 = t1961 * t833;
    (t17007, t17012, t17019, t17020)
}
