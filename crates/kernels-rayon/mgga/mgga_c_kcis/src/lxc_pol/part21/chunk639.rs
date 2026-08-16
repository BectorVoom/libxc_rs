//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 639/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk639(t3337: f64, t5062: f64, t1200: f64, t1809: f64, t388: f64, t4772: f64, t387: f64, t1187: f64, t1801: f64, t3474: f64, t3438: f64, t4984: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5063 = t3337 * t5062;
    let t5065 = t1809 * t1200;
    let t5067 = t388 * t4772;
    let t5068 = t387 * t5067;
    let t5069 = t1187 * t5068;
    let t5071 = t3474 * t1801;
    let t5073 = t3438 * t4984;
    (t5063, t5065, t5067, t5068, t5069, t5071, t5073)
}
