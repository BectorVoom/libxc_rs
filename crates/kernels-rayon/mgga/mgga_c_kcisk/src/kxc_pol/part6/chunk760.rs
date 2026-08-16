//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 760/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk760(t12566: f64, t47: f64, t12535: f64, t2921: f64, t12552: f64, t848: f64, t247: f64, t3327: f64, t242: f64, t1077: f64, t3313: f64, t3331: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15563 = t47 * t12566;
    let t15564 = t12535 * t2921;
    let t15567 = t12552 * t848;
    let t15570 = t12535 * t848;
    let t15577 = 1.0_f64 / t3327 / t247;
    let t15578 = t242 * t15577;
    let t15579 = t3313 * t1077;
    let t15580 = t15579 * t3331;
    (t15563, t15564, t15567, t15570, t15578, t15579, t15580)
}
