//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1832/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1832(t13487: f64, t86721: f64, t22960: f64, t58071: f64, t2: f64, t2752: f64, t584: f64, t868: f64, t4303: f64, t606: f64, t870: f64, t776: f64) -> (f64, f64, f64, f64, f64) {
    let t86722 = t86721 * t13487;
    let t86727 = t22960 * t58071;
    let t86730 = t2752 * t2;
    let t86732 = t86730 * t584 * t868;
    let t86746 = t606 * t4303;
    let t86753 = t870 * t2;
    let t86755 = t86753 * t584 * t776;
    (t86722, t86727, t86732, t86746, t86755)
}
