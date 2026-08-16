//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2392/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2392(t48907: f64, t48920: f64, t48934: f64, t48990: f64, t49004: f64, t49026: f64, t49042: f64, t49062: f64, t893: f64, t913: f64, t14388: f64, t2836: f64, t2842: f64) -> (f64, f64) {
    let t49068 = 1.0_f64 * t893 * (t48907 + t48920 + t48934 + t48990 + t49004 + t49026 + t49042 + t49062) * t913;
    let t49071 = 0.48245938496077605201e2_f64 * t2842 * t14388 * t2836;
    (t49068, t49071)
}
