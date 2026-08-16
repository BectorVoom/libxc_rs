//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 904/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk904(t1127: f64, t689: f64, t2427: f64, t25: f64, t677: f64, t200: f64, t709: f64, t13473: f64, t3758: f64, t1113: f64, t237: f64, t213: f64, t5001: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17950 = t689 * t1127;
    let t17957 = t2427 * t25;
    let t17958 = t677 * t17957;
    let t17959 = t200 * t1127;
    let t17960 = t17959 * t709;
    let t17964 = t3758 * t13473;
    let t17965 = t200 * t1113;
    let t17966 = t17965 * t709;
    let t17970 = t237 * t25;
    let t17971 = t3758 * t17970;
    let t17975 = t213 * t5001;
    (t17950, t17958, t17960, t17964, t17966, t17971, t17975)
}
