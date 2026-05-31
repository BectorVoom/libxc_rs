//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3261/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3261<F: Float>(t22849: F, t3957: F, t13790: F, t22020: F, t2661: F, t9934: F, t73321: F, t48152: F, t73329: F, t73341: F, t73350: F, t39419: F, t39422: F, t46297: F, t46963: F, t46970: F, t47753: F, t47760: F, t48157: F, t48159: F, t85390: F, t85391: F) -> (F, F, F, F, F, F, F, F) {
    let t85873 = t3957 * t22849;
    let t85885 = t2661 * t9934 * t22020 * t13790;
    let t85887 = F::cast_from(60.0_f64) * t73321;
    let t85888 = F::cast_from(36.0_f64) * t48152;
    let t85889 = F::cast_from(36.0_f64) * t73329;
    let t85890 = F::cast_from(0.32530743900905219526e-1_f64) * t73341;
    let t85891 = F::cast_from(3.0_f64) * t73350;
    let t85892 = t47753 + t85390 - t85391 - t47760 - t46297 - t39419 - t39422 + t85887 - t85888 - t85889 - t48157 + t48159 - t46963 + t46970 + t85890 + t85891;
    (t85873, t85885, t85887, t85888, t85889, t85890, t85891, t85892)
}
