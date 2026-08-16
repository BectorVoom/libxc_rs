//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 785/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk785(t144: f64, t2314: f64, t130: f64, t3: f64, t160: f64, t15: f64, t717: f64, t787: f64, t2440: f64, t5: f64, t88: f64, t66: f64, t728: f64) -> (f64, f64, f64, f64, f64) {
    let t9097 = t144 * t2314;
    let t9098 = t130 * t3;
    let t9099 = t9098 * t160;
    let t9102 = t15 * t717;
    let t9103 = t787 * t9102;
    let t9105 = t5 * t88 * t2440;
    let t9109 = t5 * t66 * t728;
    (t9097, t9099, t9103, t9105, t9109)
}
