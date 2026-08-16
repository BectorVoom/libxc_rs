//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 836/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk836(t11680: f64, t2563: f64, t9647: f64, t123: f64, t35439: f64, t40594: f64, t16880: f64, t35446: f64, t11894: f64, t2508: f64, t7226: f64, t7291: f64) -> (f64, f64, f64, f64) {
    let t44797 = t9647 * t11680 * t2563;
    let t44798 = 0.22430701504581487494e-2_f64 * t44797;
    let t44799 = t35439 * t123;
    let t44801 = t9647 * t44799 * t40594;
    let t44802 = 0.38452631150711121418e-2_f64 * t44801;
    let t44804 = t9647 * t16880 * t35446;
    let t44805 = 0.19226315575355560709e-2_f64 * t44804;
    let t44809 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t11894 * t7291;
    (t44798, t44802, t44805, t44809)
}
