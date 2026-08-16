//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1346/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1346(t120550: f64, t114208: f64, t114216: f64, t114285: f64, t1992: f64, t26355: f64, t114240: f64, t114242: f64, t114172: f64, t6897: f64, t7700: f64, t22674: f64, t32697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t120551 = 0.82246703342411321825e-2_f64 * t120550;
    let t120552 = 0.76763589786250567036e-1_f64 * t114208;
    let t120553 = 0.76763589786250567036e-1_f64 * t114216;
    let t120556 = 0.3289868133696452873e-1_f64 * t1992 * t114285 * t26355;
    let t120561 = 0.16449340668482264365e-1_f64 * t114240;
    let t120566 = 0.38381794893125283518e-1_f64 * t114242;
    let t120568 = t6897 * t114172 * t7700;
    let t120569 = 0.82246703342411321825e-2_f64 * t120568;
    let t120576 = t6897 * t22674 * t32697;
    (t120551, t120552, t120553, t120556, t120561, t120566, t120569, t120576)
}
