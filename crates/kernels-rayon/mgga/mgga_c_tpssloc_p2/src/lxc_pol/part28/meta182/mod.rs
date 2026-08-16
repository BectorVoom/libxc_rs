//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk890;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk891;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta182(t1454: f64, t626: f64, t1453: f64, t2331: f64, t666: f64, t1444: f64, t2341: f64, t659: f64, t2: f64, t95: f64, t584: f64, t1449: f64, t2349: f64, t662: f64, t103: f64, t100: f64, t1445: f64, t1447: f64, t657: f64, t663: f64, t92: f64, t109: f64, t656: f64, t2327: f64, t2328: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4041, t4043, t4044, t4049, t4050, t4053, t4054, t4059) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk890(t1454, t626, t1453, t2331, t666, t1444, t2341, t659, t2, t95, t584, t1449, t2349);
        let (t4063, t4067) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk891(t4059, t662, t103, t2, t584, t100, t1445, t1447, t4050, t4054, t657, t663, t92);
        let (t4068, t4072) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk892(t109, t4067, t656, t2327, t2328, t4041, t4044, t64);
    (t4043, t4044, t4049, t4050, t4053, t4054, t4059, t4063, t4067, t4068, t4072)
}
