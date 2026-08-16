//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2349/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2349(t48157: f64, t13543: f64, t699: f64, t13547: f64, t13556: f64, t13529: f64, t13533: f64, t41887: f64, t41889: f64, t43002: f64, t48122: f64, t48125: f64, t48128: f64, t48131: f64, t48134: f64, t48137: f64, t48142: f64, t48145: f64, t48148: f64, t48153: f64, t48156: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48158 = 5.0_f64 / 27.0_f64 * t48157;
    let t48159 = t699 * t13543;
    let t48161 = t699 * t13547;
    let t48163 = t699 * t13556;
    let t48165 = t699 * t13529;
    let t48167 = t699 * t13533;
    let t48169 = -8.0_f64 / 9.0_f64 * t48122 + 3.0_f64 * t48125 + t48128 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t48131 + t48134 / 18.0_f64 + 14.0_f64 / 81.0_f64 * t48137 - t48142 + 3.0_f64 * t48145 + 2.0_f64 / 9.0_f64 * t48148 + 2.0_f64 / 3.0_f64 * t41887 - t41889 / 9.0_f64 - 4.0_f64 * t48153 - t48156 + t48158 + 4.0_f64 / 3.0_f64 * t48159 + 2.0_f64 / 3.0_f64 * t48161 + 2.0_f64 / 3.0_f64 * t48163 - 2.0_f64 / 9.0_f64 * t48165 - t48167 / 9.0_f64 - t43002;
    (t48159, t48161, t48163, t48165, t48167, t48169)
}
