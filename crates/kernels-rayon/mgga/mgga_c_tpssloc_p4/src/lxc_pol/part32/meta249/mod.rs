//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1126;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1127;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1128;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1129;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1130;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1131;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1132;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta249(t6613: f64, t812: f64, t831: f64, t1899: f64, t838: f64, t234: f64, t59: f64, t240: f64, t849: f64, t6580: f64, t6582: f64, t6587: f64, t6594: f64, t6603: f64, t6607: f64, t6610: f64, t218: f64, t1903: f64, t225: f64, t1911: f64, t865: f64, t2718: f64, t1906: f64, t6547: f64, t214: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6614 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1126(t6613, t812);
        let (t6615, t6618, t6619, t6620) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1127(t6614, t831, t1899, t838, t234, t59, t240);
        let t6621 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1128(t6620, t812);
        let t6624 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1129(t6621, t849, t6580, t6582, t6587, t6594, t6603, t6607, t6610, t6615, t6618);
        let (t6625, t6627) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1130(t218, t6624, t1903, t225);
        let t6632 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1131(t1911, t865, t2718);
        let (t6636, t6637) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1132(t1906, t6547, t214, t225);
        let t6638 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1133(t234, t252);
    (t6614, t6618, t6619, t6620, t6621, t6624, t6625, t6627, t6632, t6636, t6637, t6638)
}
