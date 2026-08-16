//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1813;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta476(t5: f64, t25: f64, t265: f64, t394: f64, t24541: f64, t112: f64, t671: f64, t7408: f64, t2165: f64, t2363: f64, t23772: f64, t2116: f64, t2250: f64, t23309: f64, t40: f64, t607: f64, t7274: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t3630: f64, t7301: f64, t7300: f64, t1235: f64, t7299: f64, t7302: f64, t2123: f64, t3477: f64, t2127: f64, t23383: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24542, t24543, t24545, t24552, t24555, t24562) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1813(t5, t25, t265, t394, t24541, t112, t671, t7408, t2165, t2363, t23772, t2116, t2250, t23309, t40, t607, t7274, dens_threshold, rho0, zeta_threshold);
        let (t24563, t24564, t24567, t24568, t24571, t24574) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1814(t3630, t7301, t7300, t1235, t7299, t7302, t2123, t3477, t2127, t23383);
    (t24542, t24543, t24545, t24552, t24555, t24562, t24563, t24564, t24567, t24568, t24571, t24574)
}
