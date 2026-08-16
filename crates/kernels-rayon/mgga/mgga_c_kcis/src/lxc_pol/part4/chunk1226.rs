//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1226/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1226(t1464: f64, t15823: f64, t11881: f64, t1948: f64, t2046: f64, t3805: f64, t4170: f64, t4160: f64, t11862: f64, t5668: f64, t3797: f64, t5661: f64) -> (f64, f64, f64, f64, f64) {
    let t15824 = t1464 * t15823;
    let t15826 = t11881 * t1948;
    let t15828 = t2046 * t3805;
    let t15829 = t4170 * t15828;
    let t15830 = t4160 * t15829;
    let t15832 = t11862 * t5668;
    let t15834 = t2046 * t3797;
    let t15835 = t4170 * t15834;
    let t15836 = t5661 * t15835;
    (t15824, t15826, t15830, t15832, t15836)
}
