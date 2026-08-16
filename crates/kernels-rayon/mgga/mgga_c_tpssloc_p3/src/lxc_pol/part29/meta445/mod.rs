//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta445 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta445(t22802: f64, t22869: f64, t553: f64, t1338: f64, t6955: f64, t1352: f64, t3851: f64, t6987: f64, t3856: f64, t1372: f64, t552: f64, t1307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22870, t22871, t22873, t22874, t22877, t22879, t22881, t22882) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1754(t22802, t22869, t553, t1338, t6955, t1352, t3851, t6987, t3856, t1372, t552, t1307);
    (t22870, t22871, t22873, t22874, t22877, t22879, t22881, t22882)
}
