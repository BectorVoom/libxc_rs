//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 908/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk908<F: Float>(t9810: F, t9836: F, t852: F, t833: F, t3769: F, t6137: F, t3038: F, t8009: F, t3074: F, t8219: F, t3740: F, t851: F) -> (F, F, F, F, F, F, F) {
    let t9837 = t9810 + t9836;
    let t9838 = t9837 * t852;
    let t9840 = F::new(1.0) * t833 * t9838;
    let t9842 = F::cast_from(0.16081979498692535067e2_f64) * t6137 * t3769;
    let t9844 = F::new(4.0) * t8009 * t3038;
    let t9846 = F::cast_from(0.32163958997385070134e2_f64) * t8219 * t3074;
    let t9847 = t3740 * t851;
    (t9837, t9838, t9840, t9842, t9844, t9846, t9847)
}
