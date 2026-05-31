//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1722/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1722<F: Float>(t3970: F, t9779: F, t9765: F, t9923: F, t125: F, t1399: F, t1410: F, t3934: F, t3936: F, t3938: F, t4012: F, t4057: F, t46298: F, t46655: F, t46660: F, t46671: F, t46680: F, t46682: F, t46692: F, t46695: F, t46702: F, t46704: F, t5671: F, t5673: F, t828: F, t9628: F, t9810: F, t9826: F, t9835: F, t9840: F) -> F {
    let t46706 = t9779 * t3970;
    let t46712 = t9765 * t9923;
    let t46714 = F::cast_from(0.34299214494455789577e-2_f64) * t3934 * t3936 * t46655 * t3938 + F::cast_from(0.96037800584476210818e-1_f64) * t46660 - F::cast_from(0.20579528696673473747e-1_f64) * t5671 * t3936 * t46655 * t9835 + F::cast_from(0.77173232612525526552e-2_f64) * t5671 * t5673 * t9826 * t9840 - F::cast_from(0.73180804045370872643e-3_f64) * t46671 + F::cast_from(0.51448821741683684366e-2_f64) * t3934 * t3936 * t9826 * t9810 + F::cast_from(0.60984003371142393869e-3_f64) * t46680 + F::cast_from(0.34299214494455789577e-2_f64) * t3934 * t3936 * t46682 * t3938 - F::cast_from(0.12862205435420921092e-2_f64) * t3934 * t5673 * t9826 * t4057 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t46692 + F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t46695 + F::cast_from(0.34299214494455789577e-2_f64) * t3934 * t3936 * t125 * t9628 * t1399 + F::cast_from(0.45178982497454656791e-6_f64) * t46702 + F::cast_from(0.91464571985215438873e-3_f64) * t46704 - F::cast_from(0.13605355082800796532e0_f64) * t46706 + F::cast_from(0.12862205435420921092e-1_f64) * t1410 * t4012 * t828 * t46298 - F::cast_from(0.16262400898971305032e-1_f64) * t46712;
    t46714
}
