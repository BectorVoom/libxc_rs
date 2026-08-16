//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk813;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk814;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk815;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk816;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta164(t1216: f64, t248: f64, t3570: f64, t1213: f64, t478: f64, t483: f64, t3068: f64, t1244: f64, t1230: f64, t820: f64, t1090: f64, t1089: f64, t415: f64, t61: f64, t3243: f64, t1174: f64, t1218: f64, t1227: f64, t1232: f64, t3490: f64, t3496: f64, t3506: f64, t3511: f64, t3515: f64, t3518: f64, t3524: f64, t3527: f64, t3531: f64, t3536: f64, t3542: f64, t3543: f64, t3547: f64, t3549: f64, t3552: f64, t3557: f64, t3562: f64, t3567: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3572, t3573, t3575, t3576, t3577) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk813(t1216, t248, t3570, t1213, t478, t483, t3068, t1244);
        let t3578 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk814(t1230, t820);
        let (t3579, t3580, t3584) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk815(t1090, t1216, t3578, t1089, t415);
        let (t3585, t3587, t3590) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk816(t3584, t61, t248, t3243, t1174, t1213, t1218, t1227, t1232, t3490, t3496, t3506, t3511, t3515, t3518, t3524, t3527, t3531, t3536, t3542, t3543, t3547, t3549, t3552, t3557, t3562, t3567, t3573, t3577, t3580, t488);
    (t3572, t3573, t3575, t3576, t3577, t3578, t3579, t3580, t3584, t3585, t3587, t3590)
}
