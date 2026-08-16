//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta393 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1775;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1776;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1777;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1778;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta393(t13559: f64, t908: f64, t136: f64, t4339: f64, t690: f64, t4344: f64, t10564: f64, t13537: f64, t123: f64, t13555: f64, t2768: f64, t13528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13560, t13561, t13563) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1775(t13559, t908, t136, t4339, t690);
        let t13566 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1776(t4344, t690);
        let (t13567, t13568, t13569) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1777(t13566, t10564, t13537, t123);
        let (t13571, t13572) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1778(t13555, t2768, t123);
        let (t13574, t13575) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1779(t13528, t2768, t123);
    (t13560, t13561, t13563, t13566, t13567, t13568, t13569, t13571, t13572, t13574, t13575)
}
