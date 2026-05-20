//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2658/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2658<F: Float>(t13716: F, t221: F, t3978: F, t3979: F, t124: F, t5658: F, t3938: F, t9816: F, t9818: F, t1410: F, t1414: F, t46877: F, t46879: F, t46885: F, t46886: F, t46889: F, t46893: F, t46895: F, t46918: F, t46922: F, t48421: F, t48892: F, t48900: F, t48902: F, t48905: F, t48906: F, t48909: F, t828: F) -> (F, F) {
    let t48917 = t3978 * t3979 * t221 * t13716;
    let t48919 = t124 * t5658;
    let t48922 = t9816 * t9818 * t48919 * t3938;
    let t48926 = F::cast_from(0.30011812682648815881e-2_f64) * t48892 + F::cast_from(0.28582678745379824648e-4_f64) * t46877 + F::cast_from(0.86700792194318801432e-2_f64) * t46879 + t46885 - F::cast_from(0.91464571985215438874e-3_f64) * t46886 + F::cast_from(0.13553694749236397038e-5_f64) * t46889 + F::cast_from(0.85748036236139473944e-3_f64) * t46893 + F::cast_from(0.45732285992607719436e-2_f64) * t46895 - F::cast_from(0.12004725073059526352e-1_f64) * t48900 + F::cast_from(0.12004725073059526352e0_f64) * t48902 + t48905 + F::cast_from(0.12004725073059526352e-1_f64) * t48906 - F::cast_from(0.80328230880474379779e-6_f64) * t48909 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t1414 * t828 * t48421 - F::cast_from(0.15246000842785598468e-3_f64) * t48917 + F::cast_from(0.30492001685571196935e-3_f64) * t48922 + F::cast_from(0.13605355082800796533e0_f64) * t46918 + F::cast_from(0.15246000842785598467e-3_f64) * t46922;
    (t48919, t48926)
}
