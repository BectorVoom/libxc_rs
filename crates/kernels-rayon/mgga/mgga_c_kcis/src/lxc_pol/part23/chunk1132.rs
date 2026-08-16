//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1132/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1132(t3951: f64, t3960: f64, t3716: f64, t12229: f64, t486: f64, t506: f64, t4182: f64, t4188: f64, t12344: f64, t1502: f64, t12343: f64, t561: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37622 = t3951 * t3960;
    let t38629 = t3716 * t3716;
    let t38630 = 1.0_f64 / t38629;
    let t39052 = t486 / t12229 / t506;
    let t39296 = t4182 * t4188;
    let t39301 = t1502 * t12344;
    let t39310 = t561 / t12343 / t588;
    (t37622, t38630, t39052, t39296, t39301, t39310)
}
