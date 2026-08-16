//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk828;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk829;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta169(t2375: f64, t3684: f64, t1294: f64, t2371: f64, t2528: f64, t1284: f64, t172: f64, t763: f64, t2535: f64, t184: f64, t3681: f64, t17: f64, t1388: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3686, t3688, t3690, t3691, t3692, t3693, t3695, t3696, t3697) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk828(t2375, t3684, t1294, t2371, t2528, t1284, t172, t763, t2535, t184, t3681, t17);
        let t3698 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk829(t1388);
        let (t3700, t3701) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk830(t570);
    (t3686, t3688, t3690, t3691, t3692, t3693, t3695, t3696, t3697, t3698, t3700, t3701)
}
