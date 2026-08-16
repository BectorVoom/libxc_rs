//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 915/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk915(t1419: f64, t17020: f64, t12048: f64, t5796: f64, t1401: f64, t5808: f64, t1409: f64, t16533: f64, t1951: f64, t2642: f64, t1650: f64, t4035: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17021 = t17020 * t1419;
    let t17024 = t12048 * t5796;
    let t17027 = 0.93706135855523581992e-2_f64 * t1401 * t5808;
    let t17028 = t1409 * t16533;
    let t17037 = t1951 * t2642;
    let t17040 = t4035 * t1650;
    (t17021, t17024, t17027, t17028, t17037, t17040)
}
