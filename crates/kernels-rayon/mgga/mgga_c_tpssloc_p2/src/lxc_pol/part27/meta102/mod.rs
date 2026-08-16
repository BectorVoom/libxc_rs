//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta102 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk657;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk658;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk659;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk660;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk661;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk662;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta102(t2303: f64, t72: f64, t2245: f64, t2252: f64, t2255: f64, t2284: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64, t5: f64, t2233: f64, t2235: f64, t2240: f64, t2241: f64, t605: f64, t645: f64, t86: f64, t112: f64, t111: f64, t649: f64, t671: f64, t89: f64, t1266: f64, t107: f64, t2281: f64, t626: f64, t667: f64, t106: f64, t655: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2304, t2307) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk657(t2303, t72, t2245, t2252, t2255, t2284, t609, t629, t642, t66, t80);
        let (t2311, t2312) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk658(t5, t2233, t2235, t2240, t2241, t2307, t605, t645, t86, t112);
        let t2314 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk659(t111, t649);
        let t2319 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk660(t671);
        let (t2320, t2323, t2327, t2328, t2331) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk661(t2319, t89, t1266, t671, t107, t2281, t626, t667, t106, t655);
        let t2332 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk662(t666);
    (t2304, t2307, t2311, t2312, t2314, t2319, t2320, t2323, t2327, t2328, t2331, t2332)
}
