//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk621;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta101(t2331: f64, t2332: f64, t614: f64, t94: f64, t659: f64, t2248: f64, t95: f64, t102: f64, t662: f64, t103: f64, t100: f64, t657: f64, t660: f64, t92: f64, t96: f64, tau0: f64, t109: f64, t656: f64, t2327: f64, t2328: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2333, t2336, t2341, t2342, t2349, t2350, t2351, t2354, t2355, t2358) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk621(t2331, t2332, t614, t94, t659, t2248, t95, t102, t662, t103, t100, t657, t660, t92, t96, tau0);
        let (t2359, t2363) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk622(t109, t2358, t656, t2327, t2328, t2333, t64);
    (t2333, t2336, t2341, t2342, t2349, t2350, t2351, t2354, t2355, t2358, t2359, t2363)
}
