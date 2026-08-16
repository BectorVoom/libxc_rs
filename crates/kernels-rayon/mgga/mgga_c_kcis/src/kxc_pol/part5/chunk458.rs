//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 458/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk458(t1212: f64, t1225: f64, t1233: f64, t1668: f64, t1682: f64, t1685: f64, t1694: f64, t1823: f64, t1831: f64, t1835: f64, t187: f64, t405: f64) -> f64 {
    let t1844 = -t1668 + t1682 + t187 * (-0.3109e-1_f64 * t1823 * t405 + 1.0_f64 * t1212 * t1831 + t1668 - t1682 - 0.19751789702565206229e-1_f64 * t1685 + 0.58482233974552040708e0_f64 * t1225 * t1835) + 0.19751789702565206229e-1_f64 * t187 * t1685 - 0.58482233974552040708e0_f64 * t1233 * t1694;
    t1844
}
