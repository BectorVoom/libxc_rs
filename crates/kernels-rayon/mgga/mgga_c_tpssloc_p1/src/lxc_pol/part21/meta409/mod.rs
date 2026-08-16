//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1907;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1908;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1909;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1910;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta409(t14722: f64, t14704: f64, t11147: f64, t1409: f64, t2244: f64, t11145: f64, t123: f64, t11153: f64, t3240: f64, t3242: f64, t3966: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14723, t14724, t14725, t14726) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1907(t14722, t14704, t11147, t1409, t2244);
        let (t14727, t14728) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1908(t11145, t14726, t123);
        let (t14730, t14731) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1909(t11153, t1409, t2244);
        let (t14732, t14733) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1910(t14731, t3240, t123);
        let t14736 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1911(t3242, t3966, t607);
    (t14723, t14724, t14725, t14726, t14727, t14728, t14730, t14731, t14732, t14733, t14736)
}
