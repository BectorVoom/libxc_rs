//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta42 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk284;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk285;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk286;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk287;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk288;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk289;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta42(t315: f64, t942: f64, t880: f64, t906: f64, t323: f64, t300: f64, t134: f64, t340: f64, t344: f64, t221: f64, t339: f64, t209: f64, t338: f64, t39: f64, t119: f64, t60: f64, t270: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t943, t945, t948, t951) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk284(t315, t942, t880, t906, t323);
        let t959 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk285(t300, t315);
        let (t967, t971, t972) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk286(t134, t340, t344, t221, t339, t209, t338);
        let t973 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk287(t39, t972);
        let t974 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk288(t119, t60);
        let t976 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk289(t270, t271);
        let t977 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk290(t974, t976);
    (t943, t945, t948, t951, t959, t967, t971, t972, t973, t974, t976, t977)
}
