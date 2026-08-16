//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1000/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1000(t1203: f64, t3688: f64, t1197: f64, t3722: f64, t13064: f64, t325: f64, t12885: f64, t3725: f64, t1212: f64, t13099: f64, t12884: f64, t12888: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14728 = t3688 * t1203;
    let t14733 = t1197 * t3722;
    let t14736 = t325 * t13064;
    let t14737 = t12885 * t3725;
    let t14740 = t13099 * t1212;
    let t14743 = t325 * t12884;
    let t14744 = t12885 * t12888;
    (t14728, t14733, t14736, t14737, t14740, t14743, t14744)
}
