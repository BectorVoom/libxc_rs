//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 416/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk416(t1313: f64, t1338: f64, t1342: f64, t1357: f64, t1559: f64, t1564: f64, t1573: f64, t1577: f64, t1578: f64, t1585: f64, t187: f64, t601: f64) -> f64 {
    let t1588 = -t1313 + t1338 + t187 * (-0.3109e-1_f64 * t1559 * t601 + 1.0_f64 * t1564 * t1573 + t1313 - t1338 - 0.19751789702565206229e-1_f64 * t1342 + 0.58482233974552040708e0_f64 * t1577 * t1578) + 0.19751789702565206229e-1_f64 * t187 * t1342 - 0.58482233974552040708e0_f64 * t1585 * t1357;
    t1588
}
