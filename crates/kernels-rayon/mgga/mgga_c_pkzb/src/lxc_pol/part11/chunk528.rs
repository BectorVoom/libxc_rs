//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 528/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk528(t204: f64, t205: f64, t2739: f64, t1831: f64, t1833: f64, t2730: f64, t228: f64, t1070: f64, t663: f64) -> (f64, f64, f64, f64) {
    let t2741 = t204 * t205 * t2739;
    let t2743 = t1831 - 0.17808333333333333333e-1_f64 * t1833 - 0.17808333333333333333e-1_f64 * t2730 + 0.53425e-1_f64 * t2741;
    let t2745 = 0.621814e-1_f64 * t2743 * t228;
    let t2746 = t1070 * t663;
    (t2741, t2743, t2745, t2746)
}
