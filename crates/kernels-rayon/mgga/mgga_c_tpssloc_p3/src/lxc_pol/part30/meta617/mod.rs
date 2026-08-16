//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta617(t6695: f64, t82632: f64, t1920: f64, t2966: f64, t6699: f64, t6707: f64, t11094: f64, t6818: f64, t1958: f64, t43637: f64, t1081: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t83368, t83444, t83459, t83472, t83479, t83555) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2016(t6695, t82632, t1920, t2966, t6699, t6707, t11094, t6818, t1958, t43637, t1081, t2752);
    (t83368, t83444, t83459, t83472, t83479, t83555)
}
