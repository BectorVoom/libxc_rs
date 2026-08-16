//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 599/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk599(t143: f64, t1849: f64, t3290: f64, t682: f64, t1060: f64, t1814: f64, t1824: f64, t3293: f64, t681: f64, t4658: f64, t4684: f64, t1835: f64, t4644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5089 = t143 * t1849;
    let t5090 = t682 * t3290;
    let t5093 = t1814 * t1060;
    let t5094 = t5093 * t1824;
    let t5097 = t682 * t3293;
    let t5100 = t681 * t681;
    let t5101 = 1.0_f64 / t5100;
    let t5102 = t5101 * t4658;
    let t5105 = t1814 * t4684;
    let t5111 = t1835 * t4644;
    (t5089, t5090, t5093, t5094, t5097, t5100, t5101, t5102, t5105, t5111)
}
