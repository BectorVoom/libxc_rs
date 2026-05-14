//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1185/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1185<F: Float>(t28182: F, t922: F, t92693: F, t46978: F, t8086: F, t7772: F, t26955: F, t26960: F, t26977: F, t27070: F, t28137: F, t28190: F, t95653: F, t95658: F, t95662: F, t95666: F, t95688: F, t95727: F, t95730: F) -> (F, F) {
    let t96804 = t92693 * t28182 * t922;
    let t96812 = t46978 * t8086;
    let t96813 = t7772 * t96812;
    let t96823 = -0.23168402777777777778e-3 * t26960 * t96804 - 0.30918233506944444444e-4 * t26955 * t96804 - 0.23214722222222222222e-2 * t95653 - 0.10446625e-1 * t95658 - 0.18571777777777777777e-1 * t95662 - 0.10306077835648148148e-4 * t96813 - 0.23214722222222222222e-2 * t95666 - 0.69505208333333333334e-3 * t28190 * t26977 - 0.46429444444444444444e-2 * t95688 - 0.2782641015625e-3 * t27070 * t28137 - 0.11607361111111111111e-2 * t95727 - 0.19345601851851851852e-2 * t95730;
    (t96812, t96823)
}
