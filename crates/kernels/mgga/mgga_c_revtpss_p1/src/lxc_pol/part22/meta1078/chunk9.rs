//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3869/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3869<F: Float>(t46760: F, t46767: F, t46787: F, t46789: F, t48664: F, t48666: F, t48668: F, t48685: F, t48687: F, t48690: F, t48692: F, t46800: F, t46804: F, t46810: F, t46812: F, t46817: F, t46820: F, t46824: F, t48696: F, t48700: F, t48709: F, t48734: F) -> (F, F) {
    let t74390 = -t46760 - F::cast_from(0.30492001685571196935e-2_f64) * t48664 + F::cast_from(0.20007875121765877254e-2_f64) * t48666 + F::cast_from(0.54208002996571016772e-3_f64) * t48668 + F::cast_from(0.11337795902333997111e-1_f64) * t46767 - F::cast_from(0.16065646176094875955e-5_f64) * t46787 - F::cast_from(0.76220476654346199061e-4_f64) * t46789 + F::cast_from(35.0_f64) / F::cast_from(18.0_f64) * t48685 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t48687 - F::cast_from(0.2032800112371413129e-3_f64) * t48690 - F::cast_from(0.1219527626469539185e-2_f64) * t48692;
    let t74397 = F::cast_from(0.72286371995927450868e-4_f64) * t48696 + F::cast_from(0.72286371995927450867e-4_f64) * t48700 + t46800 + F::cast_from(0.10164000561857065645e-3_f64) * t48709 - F::cast_from(0.15246000842785598467e-3_f64) * t48734 + F::cast_from(0.90357964994909313582e-5_f64) * t46804 + t46810 - F::cast_from(0.45178982497454656791e-5_f64) * t46812 - t46817 + t46820 - t46824;
    (t74390, t74397)
}
