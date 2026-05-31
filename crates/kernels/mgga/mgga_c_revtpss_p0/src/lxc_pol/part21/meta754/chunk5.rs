//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2643/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2643<F: Float>(t46917: F, t5701: F, t14005: F, t46740: F, t5697: F, t13944: F, t1399: F, t3934: F, t3936: F, t4004: F, t4057: F, t46692: F, t46695: F, t46702: F, t46704: F, t46706: F, t46712: F, t46719: F, t46723: F, t46741: F, t46747: F, t46749: F, t48595: F, t5671: F, t5673: F, t5704: F, t9899: F) -> F {
    let t48614 = t46917 * t5701;
    let t48615 = F::cast_from(0.17006693853500995666e-1_f64) * t48614;
    let t48637 = t46740 * t14005;
    let t48638 = F::cast_from(0.40656002247428262579e-3_f64) * t48637;
    let t48645 = t46917 * t5697;
    let t48647 = -t48615 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t46692 + F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t46695 + F::cast_from(0.33884236873090992593e-6_f64) * t46702 + F::cast_from(0.45732285992607719436e-3_f64) * t46704 - F::cast_from(0.68026775414003982663e-1_f64) * t46706 - F::cast_from(0.81312004494856525156e-2_f64) * t46712 - F::cast_from(0.15246000842785598467e-2_f64) * t46719 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t5673 * t48595 * t1399 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t5673 * t13944 * t4057 + F::cast_from(0.11337795902333997111e0_f64) * t46723 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t5704 * t9899 - F::cast_from(0.32524801797942610064e-2_f64) * t46741 + t48638 + F::cast_from(0.38586616306262763275e-2_f64) * t5671 * t5673 * t13944 * t4004 - F::cast_from(0.60023625365297631762e-2_f64) * t46747 - F::cast_from(0.12004725073059526352e-1_f64) * t46749 + F::cast_from(0.68026775414003982663e-1_f64) * t48645;
    t48647
}
