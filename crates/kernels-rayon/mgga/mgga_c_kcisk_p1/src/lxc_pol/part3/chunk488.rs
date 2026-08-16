//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 488/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk488(t1390: f64, t143: f64, t3278: f64, t425: f64, t1056: f64, t1354: f64, t1364: f64, t3283: f64, t424: f64, t3593: f64, t3619: f64, t3117: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3819 = t143 * t1390;
    let t3820 = t425 * t3278;
    let t3823 = t1354 * t1056;
    let t3824 = t3823 * t1364;
    let t3827 = t425 * t3283;
    let t3830 = t424 * t424;
    let t3831 = 1.0_f64 / t3830;
    let t3832 = t3831 * t3593;
    let t3835 = t1354 * t3619;
    let t3841 = t3117 * t79;
    (t3819, t3820, t3823, t3824, t3827, t3830, t3831, t3832, t3835, t3841)
}
