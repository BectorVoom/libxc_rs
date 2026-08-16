//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta207 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk978;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk979;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk980;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta207(t2331: f64, t5464: f64, t1444: f64, t2341: f64, t5396: f64, t95: f64, t1419: f64, t1449: f64, t2349: f64, t103: f64, t100: f64, t104: f64, t1447: f64, t1450: f64, t92: f64, tau1: f64, t109: f64, t656: f64, t2327: f64, t4041: f64, t64: f64, t510: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5465, t5468, t5469, t5472, t5475, t5480, t5484, t5488) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk978(t2331, t5464, t1444, t2341, t5396, t95, t1419, t1449, t2349, t103, t100, t104, t1447, t1450, t92, tau1);
        let (t5489, t5493) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk979(t109, t5488, t656, t2327, t4041, t5465, t64);
        let t5494 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk980(t510, t5493);
    (t5465, t5468, t5469, t5472, t5475, t5480, t5484, t5488, t5489, t5493, t5494)
}
