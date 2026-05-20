//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3746/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3746<F: Float>(t17384: F, t17448: F, t17183: F, t17350: F, t20944: F, t3153: F, t372: F, t3601: F, t6587: F, t1222: F, t12809: F, t12855: F, t17355: F, t17464: F, t17646: F, t17654: F, t17693: F, t17694: F, t21004: F, t3604: F, t3611: F, t3720: F, t44190: F, t44624: F, t5308: F, t5312: F, t5373: F, t58889: F, t58897: F, t68280: F, t68295: F, t69844: F, t70933: F) -> (F, F, F) {
    let t71232 = t17448 * t17384;
    let t71238 = t17183 * t17350;
    let t71245 = t372 * t20944 * t3153;
    let t71258 = t6587 * t3601;
    let t71269 = -F::cast_from(0.3811023832717309953e-3_f64) * t71232 + F::cast_from(0.1270341277572436651e-3_f64) * t58889 - F::cast_from(0.57165357490759649296e-3_f64) * t17448 * t17646 + F::cast_from(0.3811023832717309953e-3_f64) * t58897 + F::cast_from(0.57165357490759649296e-3_f64) * t71238 * t17355 + F::cast_from(0.47637797908966374414e-3_f64) * t17693 * t17694 * t69844 + F::cast_from(0.19055119163586549765e-2_f64) * t17654 * t71245 * t44190 * t70933 - F::new(4.0) / F::new(81.0) * t5373 * t17464 + t1222 * t5312 * t68280 / F::new(108.0) - t1222 * t5308 * t68295 / F::new(72.0) - F::cast_from(0.42874018118069736972e-3_f64) * t12855 * t3720 * t71258 * t3604 + F::cast_from(0.21437009059034868486e-3_f64) * t12809 * t3720 * t71258 * t3611 + F::cast_from(0.17149607247227894789e-2_f64) * t44624 * t21004;
    (t71245, t71258, t71269)
}
