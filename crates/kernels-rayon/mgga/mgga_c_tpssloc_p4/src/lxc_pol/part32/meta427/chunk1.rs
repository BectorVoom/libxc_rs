//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1654/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1654(t19591: f64, t592: f64, t6328: f64, t11984: f64, t15880: f64, t15889: f64, t15894: f64, t19543: f64, t19574: f64, t19576: f64, t19577: f64, t19581: f64, t19588: f64, t19589: f64, t19590: f64, t3918: f64, t3919: f64, t5122: f64, t5126: f64, t5161: f64, t5187: f64, t5308: f64, t6347: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64) {
    let t19592 = 4.0_f64 * t19591;
    let t19593 = t592 * t6328;
    let t19594 = 4.0_f64 * t19593;
    let t19595 = -6.0_f64 * t19577 * t3918 * t5161 + 3.0_f64 * t3918 * t3919 * t6347 + 6.0_f64 * t3918 * t5122 * t5187 + 12.0_f64 * t5122 * t5126 * t5308 - t11984 - t15880 + t15889 - t15894 - t19543 + t19574 + t19576 - t19581 + t19588 - t19589 - t19590 + t19592 - t19594 - t9457 + t9476 + t9484;
    (t19592, t19594, t19595)
}
