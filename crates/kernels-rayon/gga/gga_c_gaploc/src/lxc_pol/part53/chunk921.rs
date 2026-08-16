//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 921/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk921(t1457: f64, t2103: f64, t43217: f64, t10867: f64, t9989: f64, t10086: f64, t10811: f64, t43508: f64, t7427: f64, t7573: f64, t326: f64, t43486: f64, t825: f64) -> (f64, f64, f64, f64, f64) {
    let t43729 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t43217;
    let t43735 = 0.25025342966295298669e1_f64 * t10867 * t1457 * t9989;
    let t43737 = 0.42900587942220512003e1_f64 * t10811 * t10086;
    let t43740 = 0.62115540045351614476e2_f64 * t7427 * t7573 * t43508;
    let t43743 = 0.18404604457881959845e2_f64 * t825 * t326 * t43486;
    (t43729, t43735, t43737, t43740, t43743)
}
