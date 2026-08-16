//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2643/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2643(t46917: f64, t5701: f64, t14005: f64, t46740: f64, t5697: f64, t13944: f64, t1399: f64, t3934: f64, t3936: f64, t4004: f64, t4057: f64, t46692: f64, t46695: f64, t46702: f64, t46704: f64, t46706: f64, t46712: f64, t46719: f64, t46723: f64, t46741: f64, t46747: f64, t46749: f64, t48595: f64, t5671: f64, t5673: f64, t5704: f64, t9899: f64) -> f64 {
    let t48614 = t46917 * t5701;
    let t48615 = 0.17006693853500995666e-1_f64 * t48614;
    let t48637 = t46740 * t14005;
    let t48638 = 0.40656002247428262579e-3_f64 * t48637;
    let t48645 = t46917 * t5697;
    let t48647 = -t48615 + 7.0_f64 / 12.0_f64 * t46692 + 35.0_f64 / 24.0_f64 * t46695 + 0.33884236873090992593e-6_f64 * t46702 + 0.45732285992607719436e-3_f64 * t46704 - 0.68026775414003982663e-1_f64 * t46706 - 0.81312004494856525156e-2_f64 * t46712 - 0.15246000842785598467e-2_f64 * t46719 - 0.64311027177104605458e-3_f64 * t3934 * t5673 * t48595 * t1399 - 0.64311027177104605458e-3_f64 * t3934 * t5673 * t13944 * t4057 + 0.11337795902333997111e0_f64 * t46723 + 0.85748036236139473944e-3_f64 * t3934 * t3936 * t5704 * t9899 - 0.32524801797942610064e-2_f64 * t46741 + t48638 + 0.38586616306262763275e-2_f64 * t5671 * t5673 * t13944 * t4004 - 0.60023625365297631762e-2_f64 * t46747 - 0.12004725073059526352e-1_f64 * t46749 + 0.68026775414003982663e-1_f64 * t48645;
    t48647
}
