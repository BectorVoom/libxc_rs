//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 494/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk494(t1564: f64, t1577: f64, t1585: f64, t187: f64, t1895: f64, t1909: f64, t1912: f64, t1921: f64, t2072: f64, t2080: f64, t2084: f64, t601: f64) -> f64 {
    let t2093 = -t1895 + t1909 + t187 * (-0.3109e-1_f64 * t2072 * t601 + 1.0_f64 * t1564 * t2080 + t1895 - t1909 - 0.19751789702565206229e-1_f64 * t1912 + 0.58482233974552040708e0_f64 * t1577 * t2084) + 0.19751789702565206229e-1_f64 * t187 * t1912 - 0.58482233974552040708e0_f64 * t1585 * t1921;
    t2093
}
