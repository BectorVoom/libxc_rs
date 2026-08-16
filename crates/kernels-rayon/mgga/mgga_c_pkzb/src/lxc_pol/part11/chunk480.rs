//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 480/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk480(t154: f64, t386: f64, t486: f64, t385: f64, t405: f64, t67: f64) -> (f64, f64, f64) {
    let t2344 = t154 * t486 * t386;
    let t2346 = t385 * t2344 / 432.0_f64;
    let t2347 = t67 * t405;
    (t2344, t2346, t2347)
}
