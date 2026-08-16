//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1234/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1234(t1251: f64, t20666: f64, t15555: f64, t15576: f64, t20632: f64, t20635: f64, t20639: f64, t20642: f64, t20645: f64, t20649: f64, t20654: f64, t20658: f64, t20662: f64, t3490: f64, t3514: f64, t6767: f64) -> f64 {
    let t20667 = t1251 * t20666;
    let t20669 = t15555 / 432.0_f64 + t3514 * t20632 / 96.0_f64 - t3514 * t20635 / 72.0_f64 - t3514 * t20639 / 576.0_f64 - t3514 * t20642 / 288.0_f64 + t3514 * t20645 / 432.0_f64 + t15576 + t1251 * t20649 / 576.0_f64 - t1251 * t20654 / 32.0_f64 + t1251 * t20658 / 48.0_f64 + t20662 / 1296.0_f64 + t3490 * t6767 / 108.0_f64 - t20667 / 864.0_f64;
    t20669
}
