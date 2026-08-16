//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3164/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3164<F: Float>(t3617: F, t372: F, t6628: F, t20973: F, t5391: F, t5381: F, t12916: F, t24735: F, t5331: F, t12809: F, t17351: F, t17661: F, t21222: F, t21246: F, t21267: F, t21275: F, t3611: F, t3720: F, t44264: F, t44510: F, t5047: F, t70091: F, t70102: F, t70959: F, t82321: F) -> (F, F) {
    let t83125 = t372 * t3617 * t6628;
    let t83130 = t5391 * t20973;
    let t83136 = t5381 * t20973;
    let t83143 = t5331 * t12916 * t24735;
    let t83145 = -F::cast_from(0.45732285992607719436e-2_f64) * t70091 - F::cast_from(0.57165357490759649296e-3_f64) * t70102 + F::cast_from(0.85748036236139473947e-3_f64) * t44510 * t17661 * t21222 + F::cast_from(0.63517063878621832551e-4_f64) * t44264 - F::cast_from(0.7145669686344956162e-3_f64) * t17351 * t83125 * t3611 * t5047 + F::cast_from(0.15244095330869239812e-2_f64) * t83130 + F::cast_from(0.12862205435420921092e-2_f64) * t21275 * t21246 + F::cast_from(0.20579528696673473746e-1_f64) * t70959 * t21267 - F::cast_from(0.28582678745379824648e-3_f64) * t83136 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t82321 * t3611 - F::cast_from(0.42874018118069736972e-3_f64) * t83143;
    (t83125, t83145)
}
