//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1213/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1213(t157: f64, t15951: f64, t15966: f64, t182: f64, t1787: f64, t2516: f64, t17: f64, t12097: f64, t12100: f64, t12111: f64, t12120: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15968 = (t15951 + t15966) * t157;
    let t15970 = 0.19751673498613801407e-1_f64 * t15968 * t182;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15973 = 0.4883052614935078681e-3_f64 * t12097;
    let t15974 = 0.18311447306006545054e-3_f64 * t12100;
    let t15975 = 0.21687162600603479684e-1_f64 * t12111;
    let t15976 = 4.0_f64 * t12120;
    let t15977 = t15968 * t184;
    (t15970, t15972, t15973, t15974, t15975, t15976, t15977)
}
