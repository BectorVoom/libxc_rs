//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 901/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk901(t2568: f64, t8561: f64, t126: f64, t691: f64, t2314: f64, t4: f64, t789: f64, t15: f64, t26: f64, t92: f64, t160: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8562 = t2568 * t8561;
    let t8565 = t126 * t691;
    let t8566 = t8565 * t2314;
    let t8567 = t789 * t4;
    let t8572 = 1.0_f64 / t15 / t26 / 4.0_f64;
    let t8573 = t8572 * t92;
    let t8574 = t3 * t160;
    (t8562, t8565, t8566, t8567, t8572, t8573, t8574)
}
