//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 949/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk949(t2977: f64, t484: f64, t5042: f64, t691: f64, t276: f64, t40: f64, t4027: f64, t1284: f64, t228: f64, t1292: f64, t3937: f64, t5351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15018 = t2977 * t484;
    let t15043 = t5042 * t691;
    let t15050 = t40 * t4027 * t276;
    let t15072 = 16.0_f64 * t1284 * t228;
    let t15095 = 16.0_f64 * t1292 * t228;
    let t15106 = t3937 * t5351;
    (t15018, t15043, t15050, t15072, t15095, t15106)
}
