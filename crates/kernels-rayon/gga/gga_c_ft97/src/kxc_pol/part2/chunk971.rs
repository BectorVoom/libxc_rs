//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 971/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk971(t10649: f64, t10797: f64, t14718: f64, t14892: f64, t14899: f64, t14949: f64, t14951: f64, t15058: f64, t15062: f64, t15065: f64, t15069: f64, t14927: f64, t14939: f64, t14947: f64) -> f64 {
    let t15071 = -22.0_f64 / 9.0_f64 * t14718 - t10649 - t14892 - t14949 + 2.0_f64 / 3.0_f64 * t14899 + t14951 + t15058 / 2.0_f64 - t10797 - t15062 / 2.0_f64 - t15065 / 4.0_f64 + 3.0_f64 / 8.0_f64 * t15069;
    let t15073 = t14927 + t14939 + t14947 + t15071;
    t15073
}
