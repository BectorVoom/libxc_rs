//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1940/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1940(t232: f64, t25119: f64, t58557: f64, t815: f64, t22690: f64, t5527: f64, t81792: f64, t841: f64, t16805: f64, t1898: f64, t249: f64, t236: f64, t5584: f64) -> (f64, f64, f64, f64) {
    let t98770 = t25119 * t815 * t58557 * t232;
    let t98774 = t81792 * t22690 * t841 * t5527;
    let t98777 = t16805 * t1898 * t249;
    let t98779 = t236 * t5584;
    (t98770, t98774, t98777, t98779)
}
