//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 649/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk649(t716: f64, t9014: f64, t736: f64, t1755: f64, t8780: f64, t735: f64, t734: f64, t2580: f64, t7320: f64, t2560: f64, t2568: f64, t2572: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9015 = t9014 * t716;
    let t9016 = t9015 * sigma2;
    let t9017 = t9016 * t736;
    let t9019 = t1755 * t8780;
    let t9020 = t735 * t9019;
    let t9021 = t734 * t9020;
    let t9023 = t7320 * t2580;
    let t9025 = t2560 * t2568;
    let t9027 = t2560 * t2572;
    (t9016, t9017, t9019, t9020, t9021, t9023, t9025, t9027)
}
