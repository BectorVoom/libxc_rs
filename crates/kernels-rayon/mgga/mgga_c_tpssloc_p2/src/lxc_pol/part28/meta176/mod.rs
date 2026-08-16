//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta176 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk853;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk854;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk855;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk856;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk857;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk858;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta176(t1285: f64, t588: f64, t1287: f64, t2423: f64, t3686: f64, t3697: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3828: f64, t3830: f64, t3832: f64, t225: f64, t3817: f64, t1365: f64, t68: f64, t3734: f64, t1347: f64, t3719: f64, t1345: f64, t1348: f64, t546: f64, t548: f64, t550: f64, t1343: f64, t820: f64, t3791: f64, t248: f64, t2691: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3833, t3834, t3836, t3837) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk853(t1285, t588, t1287, t2423, t3686, t3697, t3819, t3821, t3823, t3825, t3828, t3830, t3832);
        let (t3839, t3844, t3847, t3850) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk854(t225, t3817, t3837, t1365, t68, t3734, t1347, t3719, t1345, t1348, t546, t548);
        let t3851 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk855(t3850, t550);
        let t3853 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk856(t1343, t3851, t820);
        let t3856 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk857(t3791, t550);
        let t3858 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk858(t1343, t3856, t820);
        let t3862 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk859(t248, t2691, t557);
    (t3833, t3834, t3836, t3839, t3844, t3847, t3850, t3851, t3853, t3856, t3858, t3862)
}
