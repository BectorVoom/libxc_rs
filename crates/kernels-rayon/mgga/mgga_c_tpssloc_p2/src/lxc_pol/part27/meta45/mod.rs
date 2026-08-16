//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta45 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk323;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk324;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk325;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk326;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta45(t154: f64, t676: f64, t268: f64, t271: f64, t376: f64, t632: f64, t607: f64, t123: f64, t291: f64, t287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t878, t880) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk323(t154, t676, t268, t271);
        let (t881, t882) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk324(t880, t154, t376);
        let t883 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk325(t632);
        let t884 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk326(t607, t883);
        let (t885, t886, t888, t890, t891, t892) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk327(t882, t884, t123, t881, t291, t287);
    (t878, t880, t881, t882, t883, t884, t885, t886, t888, t890, t891, t892)
}
