//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk724;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk725;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta141(t1284: f64, t750: f64, t17: f64, t1285: f64, t592: f64, t1287: f64, t588: f64, t1365: f64, t68: f64, t248: f64, t2691: f64, t557: f64, t555: f64, t1361: f64, t835: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3826, t3827, t3829, t3832, t3833, t3836, t3843, t3862) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk724(t1284, t750, t17, t1285, t592, t1287, t588, t1365, t68, t248, t2691, t557);
        let (t3864, t3865, t3866) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk725(t3862, t555, t1361, t835, t1336);
    (t3826, t3827, t3829, t3832, t3833, t3836, t3843, t3862, t3864, t3865, t3866)
}
