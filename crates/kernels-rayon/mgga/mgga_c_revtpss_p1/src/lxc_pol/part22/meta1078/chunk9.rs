//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3869/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3869(t46760: f64, t46767: f64, t46787: f64, t46789: f64, t48664: f64, t48666: f64, t48668: f64, t48685: f64, t48687: f64, t48690: f64, t48692: f64, t46800: f64, t46804: f64, t46810: f64, t46812: f64, t46817: f64, t46820: f64, t46824: f64, t48696: f64, t48700: f64, t48709: f64, t48734: f64) -> (f64, f64) {
    let t74390 = -t46760 - 0.30492001685571196935e-2_f64 * t48664 + 0.20007875121765877254e-2_f64 * t48666 + 0.54208002996571016772e-3_f64 * t48668 + 0.11337795902333997111e-1_f64 * t46767 - 0.16065646176094875955e-5_f64 * t46787 - 0.76220476654346199061e-4_f64 * t46789 + 35.0_f64 / 18.0_f64 * t48685 - 7.0_f64 / 24.0_f64 * t48687 - 0.2032800112371413129e-3_f64 * t48690 - 0.1219527626469539185e-2_f64 * t48692;
    let t74397 = 0.72286371995927450868e-4_f64 * t48696 + 0.72286371995927450867e-4_f64 * t48700 + t46800 + 0.10164000561857065645e-3_f64 * t48709 - 0.15246000842785598467e-3_f64 * t48734 + 0.90357964994909313582e-5_f64 * t46804 + t46810 - 0.45178982497454656791e-5_f64 * t46812 - t46817 + t46820 - t46824;
    (t74390, t74397)
}
