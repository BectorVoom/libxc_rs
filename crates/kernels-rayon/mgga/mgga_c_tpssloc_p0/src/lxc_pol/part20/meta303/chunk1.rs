//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1545/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1545(t11185: f64, t3316: f64, t3311: f64, t419: f64, t409: f64) -> (f64, f64, f64) {
    let t11187 = 0.48245938496077605201e2_f64 * t11185 * t3316;
    let t11189 = 1.0_f64 / t3311 / t419;
    let t11190 = t409 * t11189;
    (t11187, t11189, t11190)
}
