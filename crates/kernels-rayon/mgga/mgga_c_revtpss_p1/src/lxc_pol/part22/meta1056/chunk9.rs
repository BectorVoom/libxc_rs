//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3746/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3746(t17384: f64, t17448: f64, t17183: f64, t17350: f64, t20944: f64, t3153: f64, t372: f64, t3601: f64, t6587: f64, t1222: f64, t12809: f64, t12855: f64, t17355: f64, t17464: f64, t17646: f64, t17654: f64, t17693: f64, t17694: f64, t21004: f64, t3604: f64, t3611: f64, t3720: f64, t44190: f64, t44624: f64, t5308: f64, t5312: f64, t5373: f64, t58889: f64, t58897: f64, t68280: f64, t68295: f64, t69844: f64, t70933: f64) -> (f64, f64, f64) {
    let t71232 = t17448 * t17384;
    let t71238 = t17183 * t17350;
    let t71245 = t372 * t20944 * t3153;
    let t71258 = t6587 * t3601;
    let t71269 = -0.3811023832717309953e-3_f64 * t71232 + 0.1270341277572436651e-3_f64 * t58889 - 0.57165357490759649296e-3_f64 * t17448 * t17646 + 0.3811023832717309953e-3_f64 * t58897 + 0.57165357490759649296e-3_f64 * t71238 * t17355 + 0.47637797908966374414e-3_f64 * t17693 * t17694 * t69844 + 0.19055119163586549765e-2_f64 * t17654 * t71245 * t44190 * t70933 - 4.0_f64 / 81.0_f64 * t5373 * t17464 + t1222 * t5312 * t68280 / 108.0_f64 - t1222 * t5308 * t68295 / 72.0_f64 - 0.42874018118069736972e-3_f64 * t12855 * t3720 * t71258 * t3604 + 0.21437009059034868486e-3_f64 * t12809 * t3720 * t71258 * t3611 + 0.17149607247227894789e-2_f64 * t44624 * t21004;
    (t71245, t71258, t71269)
}
