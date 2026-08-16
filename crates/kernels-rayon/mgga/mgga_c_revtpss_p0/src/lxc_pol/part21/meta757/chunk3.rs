//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2658/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2658(t13716: f64, t221: f64, t3978: f64, t3979: f64, t124: f64, t5658: f64, t3938: f64, t9816: f64, t9818: f64, t1410: f64, t1414: f64, t46877: f64, t46879: f64, t46885: f64, t46886: f64, t46889: f64, t46893: f64, t46895: f64, t46918: f64, t46922: f64, t48421: f64, t48892: f64, t48900: f64, t48902: f64, t48905: f64, t48906: f64, t48909: f64, t828: f64) -> (f64, f64) {
    let t48917 = t3978 * t3979 * t221 * t13716;
    let t48919 = t124 * t5658;
    let t48922 = t9816 * t9818 * t48919 * t3938;
    let t48926 = 0.30011812682648815881e-2_f64 * t48892 + 0.28582678745379824648e-4_f64 * t46877 + 0.86700792194318801432e-2_f64 * t46879 + t46885 - 0.91464571985215438874e-3_f64 * t46886 + 0.13553694749236397038e-5_f64 * t46889 + 0.85748036236139473944e-3_f64 * t46893 + 0.45732285992607719436e-2_f64 * t46895 - 0.12004725073059526352e-1_f64 * t48900 + 0.12004725073059526352e0_f64 * t48902 + t48905 + 0.12004725073059526352e-1_f64 * t48906 - 0.80328230880474379779e-6_f64 * t48909 - 0.85748036236139473944e-3_f64 * t1410 * t1414 * t828 * t48421 - 0.15246000842785598468e-3_f64 * t48917 + 0.30492001685571196935e-3_f64 * t48922 + 0.13605355082800796533e0_f64 * t46918 + 0.15246000842785598467e-3_f64 * t46922;
    (t48919, t48926)
}
