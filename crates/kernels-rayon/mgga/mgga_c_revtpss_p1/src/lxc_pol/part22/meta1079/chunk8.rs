//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3878/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3878(t22021: f64, t808: f64, t9845: f64, t46879: f64, t46885: f64, t46886: f64, t46889: f64, t46895: f64, t48892: f64, t48900: f64, t48902: f64, t48904: f64, t48906: f64, t48909: f64) -> f64 {
    let t74522 = t9845 * t808 * t22021;
    let t74527 = 0.20007875121765877254e-2_f64 * t48892 + 0.57800528129545867622e-2_f64 * t46879 + t46885 - 0.30488190661738479624e-3_f64 * t46886 + 0.90357964994909313584e-6_f64 * t46889 + 0.15244095330869239812e-2_f64 * t46895 - 0.80031500487063509016e-2_f64 * t48900 + 0.80031500487063509016e-1_f64 * t48902 + 0.25410001404642664112e-5_f64 * t74522 + 0.2168320119862840671e-2_f64 * t48904 + 0.80031500487063509015e-2_f64 * t48906 - 0.16065646176094875956e-5_f64 * t48909;
    t74527
}
