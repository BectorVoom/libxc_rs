//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1345/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1345(t28182: f64, t922: f64, t92693: f64, t46978: f64, t8086: f64, t7772: f64, t26955: f64, t26960: f64, t26977: f64, t27070: f64, t28137: f64, t28190: f64, t95653: f64, t95658: f64, t95662: f64, t95666: f64, t95688: f64, t95727: f64, t95730: f64) -> (f64, f64) {
    let t96804 = t92693 * t28182 * t922;
    let t96812 = t46978 * t8086;
    let t96813 = t7772 * t96812;
    let t96823 = -0.23168402777777777778e-3_f64 * t26960 * t96804 - 0.30918233506944444444e-4_f64 * t26955 * t96804 - 0.23214722222222222222e-2_f64 * t95653 - 0.10446625e-1_f64 * t95658 - 0.18571777777777777777e-1_f64 * t95662 - 0.10306077835648148148e-4_f64 * t96813 - 0.23214722222222222222e-2_f64 * t95666 - 0.69505208333333333334e-3_f64 * t28190 * t26977 - 0.46429444444444444444e-2_f64 * t95688 - 0.2782641015625e-3_f64 * t27070 * t28137 - 0.11607361111111111111e-2_f64 * t95727 - 0.19345601851851851852e-2_f64 * t95730;
    (t96812, t96823)
}
