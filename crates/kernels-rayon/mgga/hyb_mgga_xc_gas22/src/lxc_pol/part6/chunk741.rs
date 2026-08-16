//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 741/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk741(t3827: f64, t54: f64, t3844: f64, t588: f64, t57: f64, t592: f64, t60: f64, t596: f64, t63: f64, t600: f64, t66: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3847 = t54 * t3827;
    let t3849 = t588 * t3844;
    let t3851 = t57 * t3827;
    let t3853 = t592 * t3844;
    let t3855 = t60 * t3827;
    let t3857 = t596 * t3844;
    let t3859 = t63 * t3827;
    let t3861 = t600 * t3844;
    let t3863 = t66 * t3827;
    let t3865 = t604 * t3844;
    (t3847, t3849, t3851, t3853, t3855, t3857, t3859, t3861, t3863, t3865)
}
