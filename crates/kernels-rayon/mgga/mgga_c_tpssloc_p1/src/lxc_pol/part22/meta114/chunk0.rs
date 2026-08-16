//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 773/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk773(t281: f64, t2820: f64, t415: f64, t1114: f64, t699: f64, t1176: f64, t241: f64) -> (f64, f64, f64, f64) {
    let t3293 = t281 * t2820 * t415;
    let t3294 = 0.13692777777777777778e0_f64 * t3293;
    let t3295 = t699 * t1114;
    let t3297 = t241 * t1176;
    (t3293, t3294, t3295, t3297)
}
