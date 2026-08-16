//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2246/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2246(t16944: f64, t25119: f64, t841: f64, t23083: f64, t28372: f64, t28395: f64, t81782: f64, t81783: f64, t5587: f64, t81803: f64, t1512: f64, t87295: f64) -> (f64, f64, f64, f64, f64) {
    let t98744 = t25119 * t841 * t16944;
    let t98746 = t23083 * t28372;
    let t98750 = t81782 * t81783 * t28395;
    let t98752 = t81803 * t5587;
    let t98754 = t87295 * t1512;
    (t98744, t98746, t98750, t98752, t98754)
}
