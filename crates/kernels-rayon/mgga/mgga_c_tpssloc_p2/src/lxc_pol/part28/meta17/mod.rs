//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta17 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk126;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk127;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk128;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk129;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta17(t273: f64, t276: f64, t279: f64, t285: f64, t275: f64, t148: f64, t154: f64, t157: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t287, t290, t291) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk126(t273, t276, t279, t285);
        let (t293, t300) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk127(t275, t291, t148, t154, t157, zeta_threshold);
        let t302 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk128(t273);
        let (t307, t310, t311) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk129(t273, t276, t279, t285);
        let t315 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk130(t273);
    (t287, t290, t291, t293, t300, t302, t307, t310, t311, t315)
}
