//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 753/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk753(t2244: f64, t3146: f64, t974: f64, t2775: f64, t976: f64, t1005: f64, t1036: f64, t221: f64, t2965: f64, t339: f64, t964: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3147 = t3146 * t2244;
    let t3148 = t974 * t3147;
    let t3151 = t976 * t2775;
    let t3152 = t3151 * t2244;
    let t3153 = t974 * t3152;
    let t3156 = t1005 * t1036;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / 432.0_f64;
    let t3163 = t964 * t995;
    (t3147, t3148, t3152, t3153, t3156, t3158, t3160, t3163)
}
