//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta49 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk347;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk348;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk349;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk350;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk351;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta49(t270: f64, t271: f64, t974: f64, t344: f64, t883: f64, t607: f64, t906: f64, t910: f64, t340: f64, t343: f64, t346: f64, t964: f64, t971: f64, t973: f64, t381: f64, t221: f64, t967: f64, t339: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t976 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk347(t270, t271);
        let t977 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk348(t974, t976);
        let t978 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk349(t344, t883);
        let (t979, t980, t984) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk350(t607, t978, t977, t906, t910);
        let (t986, t990) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk351(t340, t984, t343, t974, t346, t964, t971, t973, t980);
        let (t991, t995, t997, t998) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk352(t381, t990, t221, t967, t339, t883, t976);
    (t976, t977, t978, t979, t984, t986, t990, t991, t995, t997, t998)
}
