//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1105/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1105(t6635: f64, t6644: f64, t2047: f64, t814: f64) -> (f64, f64, f64) {
    let t7095 = 0.38381794893125283518e-1_f64 * t6635;
    let t7097 = 0.82246703342411321825e-2_f64 * t6644;
    let t7101 = t814 * t2047;
    (t7095, t7097, t7101)
}
