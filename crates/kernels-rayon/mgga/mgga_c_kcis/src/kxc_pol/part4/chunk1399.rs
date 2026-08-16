//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1399/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1399(t18148: f64, t18152: f64, t18156: f64, t18160: f64, t18164: f64, t18170: f64, t18174: f64, t18178: f64, t18179: f64, t4439: f64, t4447: f64, t4459: f64, t4465: f64, t6141: f64) -> f64 {
    let t18182 = -t18148 - t6141 * t4459 / 36.0_f64 + t18152 + t6141 * t4465 / 72.0_f64 - t4439 * t18156 / 216.0_f64 + t4439 * t18160 / 144.0_f64 - t18164 / 2592.0_f64 + t6141 * t4447 / 108.0_f64 - t18170 - t18174 + t18178 + t4439 * t18179 / 432.0_f64;
    t18182
}
