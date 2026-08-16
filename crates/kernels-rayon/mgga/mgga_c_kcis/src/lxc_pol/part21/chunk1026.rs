//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1026/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1026(t15573: f64, t5331: f64, t1251: f64, t10996: f64, t11086: f64, t15531: f64, t15535: f64, t15541: f64, t15547: f64, t15549: f64, t15555: f64, t15558: f64, t15563: f64, t15570: f64, t1847: f64, t3490: f64, t3514: f64, t5316: f64, t5326: f64, t5332: f64) -> f64 {
    let t15574 = t15573 * t5331;
    let t15576 = t1251 * t15574 / 144.0_f64;
    let t15577 = -t3514 * t15531 / 432.0_f64 - t3514 * t15535 / 72.0_f64 + t11086 * t5326 / 108.0_f64 + t3514 * t15541 / 288.0_f64 + 11.0_f64 / 648.0_f64 * t10996 * t1847 - t15547 - t15549 / 2592.0_f64 - t3490 * t5316 / 54.0_f64 - 7.0_f64 / 864.0_f64 * t15555 + t1251 * t15558 / 96.0_f64 - t1251 * t15563 / 32.0_f64 - t3490 * t5332 / 18.0_f64 + t1251 * t15570 / 48.0_f64 + t15576;
    t15577
}
