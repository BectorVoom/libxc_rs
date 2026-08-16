//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1127/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1127(t26271: f64, t80779: f64, t22764: f64, t5234: f64, t3862: f64, t7715: f64, t26245: f64, t80791: f64, t80836: f64, t80783: f64, t22760: f64, t1827: f64, t80914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t91206 = t80779 * t26271;
    let t91285 = t5234 * t22764;
    let t91305 = t7715 * t3862;
    let t91312 = t80791 * t26245;
    let t91323 = t80836 * t26271;
    let t91346 = t80783 * t26245;
    let t91388 = t5234 * t22760;
    let t91394 = t80914 * t1827;
    (t91206, t91285, t91305, t91312, t91323, t91346, t91388, t91394)
}
