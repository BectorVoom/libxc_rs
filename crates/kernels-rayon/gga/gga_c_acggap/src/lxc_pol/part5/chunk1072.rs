//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1072/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1072(t3088: f64, t4166: f64, t4183: f64, t3378: f64, t4176: f64, t3077: f64, t4163: f64, t1035: f64, t1647: f64, t3044: f64, t1655: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t19048 = t3088 * t4166 * t4183;
    let t19053 = t3378 * t4176;
    let t19060 = t3077 * t4163;
    let t19074 = t1035 * t1647 * t3044;
    let t19082 = t848 * t1655;
    (t19048, t19053, t19060, t19074, t19082)
}
