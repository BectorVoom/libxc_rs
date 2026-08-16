//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1631/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1631(t13487: f64, t23788: f64, t1081: f64, t776: f64, t2553: f64, t28: f64, t2749: f64, t868: f64, t2745: f64, t12461: f64, t3698: f64, t2039: f64, t3652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23789 = t23788 * t13487;
    let t23792 = t1081 * t776;
    let t23796 = t28 * t2553;
    let t23807 = t28 * t2749;
    let t23810 = t1081 * t868;
    let t23813 = t28 * t2745;
    let t23857 = t12461 * t3698;
    let t23909 = t3652 * t2039;
    (t23789, t23792, t23796, t23807, t23810, t23813, t23857, t23909)
}
