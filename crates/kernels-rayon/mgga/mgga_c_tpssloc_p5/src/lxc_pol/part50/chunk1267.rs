//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1267/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1267(t114097: f64, t114105: f64, t1985: f64, t1998: f64, t214: f64, t26328: f64, t32749: f64, t6883: f64, t1824: f64, t8470: f64, t32748: f64, t6897: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120506 = 0.82246703342411321825e-2_f64 * t114097;
    let t120507 = 0.38381794893125283518e-1_f64 * t114105;
    let t120513 = 0.16449340668482264365e-1_f64 * t1985 * t214 * t1998 * t26328;
    let t120514 = t6883 * t32749;
    let t120515 = 0.38381794893125283518e-1_f64 * t120514;
    let t120516 = t8470 * t1824;
    let t120521 = t6897 * t794 * t32748;
    (t120506, t120507, t120513, t120515, t120516, t120521)
}
