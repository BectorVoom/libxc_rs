//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3878/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3878<F: Float>(t22021: F, t808: F, t9845: F, t46879: F, t46885: F, t46886: F, t46889: F, t46895: F, t48892: F, t48900: F, t48902: F, t48904: F, t48906: F, t48909: F) -> F {
    let t74522 = t9845 * t808 * t22021;
    let t74527 = F::cast_from(0.20007875121765877254e-2_f64) * t48892 + F::cast_from(0.57800528129545867622e-2_f64) * t46879 + t46885 - F::cast_from(0.30488190661738479624e-3_f64) * t46886 + F::cast_from(0.90357964994909313584e-6_f64) * t46889 + F::cast_from(0.15244095330869239812e-2_f64) * t46895 - F::cast_from(0.80031500487063509016e-2_f64) * t48900 + F::cast_from(0.80031500487063509016e-1_f64) * t48902 + F::cast_from(0.25410001404642664112e-5_f64) * t74522 + F::cast_from(0.2168320119862840671e-2_f64) * t48904 + F::cast_from(0.80031500487063509015e-2_f64) * t48906 - F::cast_from(0.16065646176094875956e-5_f64) * t48909;
    t74527
}
