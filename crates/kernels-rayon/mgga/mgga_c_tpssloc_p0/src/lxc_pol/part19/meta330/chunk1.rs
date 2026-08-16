//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1178/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1178(t3736: f64, t40018: f64, t12012: f64, t12220: f64, t16101: f64, t210: f64, t213: f64, t214: f64, t221: f64, t3719: f64, t3733: f64, t3734: f64, t39622: f64, t40343: f64, t40347: f64, t40350: f64, t40351: f64, t40356: f64, t40360: f64, t40366: f64, t40372: f64, t40376: f64, t5195: f64) -> f64 {
    let t40387 = t40018 * t3736;
    let t40389 = -t40343 + t40347 + t40350 - 0.79999999999999999997e-1_f64 * t40351 - 0.29999999999999999998e-1_f64 * t40356 + 0.99999999999999999996e-2_f64 * t40360 + 0.19999999999999999999e-1_f64 * t5195 * t221 * t12220 * t12012 - 0.13999999999999999999e0_f64 * t40366 + 0.11111111111111111111e-2_f64 * t40372 - 0.29999999999999999998e-1_f64 * t40376 - 0.11999999999999999999e0_f64 * t16101 * t221 * t213 * t3734 * t3719 + 0.14999999999999999999e-1_f64 * t3733 * t210 * t214 * t39622 + 0.23333333333333333332e0_f64 * t40387;
    t40389
}
