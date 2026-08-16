//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2082/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2082(t25: f64, t40772: f64, t1530: f64, t2749: f64, t1408: f64, t2752: f64, t13487: f64, t22960: f64, t58071: f64, t2: f64, t584: f64, t868: f64) -> (f64, f64, f64, f64, f64) {
    let t86716 = t40772 * t25;
    let t86717 = t1530 * t2749;
    let t86718 = t86716 * t86717;
    let t86721 = t2752 * t1408;
    let t86722 = t86721 * t13487;
    let t86727 = t22960 * t58071;
    let t86730 = t2752 * t2;
    let t86732 = t86730 * t584 * t868;
    (t86717, t86718, t86722, t86727, t86732)
}
