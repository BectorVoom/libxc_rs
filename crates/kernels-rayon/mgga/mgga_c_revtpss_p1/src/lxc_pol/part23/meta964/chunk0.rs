//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3261/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3261(t22849: f64, t3957: f64, t13790: f64, t22020: f64, t2661: f64, t9934: f64, t73321: f64, t48152: f64, t73329: f64, t73341: f64, t73350: f64, t39419: f64, t39422: f64, t46297: f64, t46963: f64, t46970: f64, t47753: f64, t47760: f64, t48157: f64, t48159: f64, t85390: f64, t85391: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t85873 = t3957 * t22849;
    let t85885 = t2661 * t9934 * t22020 * t13790;
    let t85887 = 60.0_f64 * t73321;
    let t85888 = 36.0_f64 * t48152;
    let t85889 = 36.0_f64 * t73329;
    let t85890 = 0.32530743900905219526e-1_f64 * t73341;
    let t85891 = 3.0_f64 * t73350;
    let t85892 = t47753 + t85390 - t85391 - t47760 - t46297 - t39419 - t39422 + t85887 - t85888 - t85889 - t48157 + t48159 - t46963 + t46970 + t85890 + t85891;
    (t85873, t85885, t85887, t85888, t85889, t85890, t85891, t85892)
}
