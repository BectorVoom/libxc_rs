//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1151/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1151(t1512: f64, t87261: f64, t23083: f64, t28372: f64, t28395: f64, t81782: f64, t81783: f64, t22690: f64, t5527: f64, t81792: f64, t841: f64, t236: f64, t5584: f64) -> (f64, f64, f64, f64, f64) {
    let t98738 = t87261 * t1512;
    let t98746 = t23083 * t28372;
    let t98750 = t81782 * t81783 * t28395;
    let t98774 = t81792 * t22690 * t841 * t5527;
    let t98779 = t236 * t5584;
    (t98738, t98746, t98750, t98774, t98779)
}
