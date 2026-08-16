//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1784/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1784(t19660: f64, t5250: f64, t12171: f64, t6388: f64, t3901: f64, t6415: f64, t11984: f64, t15880: f64, t15889: f64, t15894: f64, t19543: f64, t19574: f64, t19576: f64, t19581: f64, t19588: f64, t19589: f64, t19590: f64, t19592: f64, t19594: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64, f64) {
    let t19661 = t19660 * t5250;
    let t19668 = t12171 * t6388;
    let t19674 = t3901 * t6415;
    let t19676 = -t19543 - t9457 + t19574 + t19576 + t9476 + t9484 - t19581 - t15880 + t19588 + t15889 - t19589 - t15894 - t19590 - t11984 + t19592 - t19594;
    (t19661, t19668, t19674, t19676)
}
