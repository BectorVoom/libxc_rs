//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2689/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2689(t118: f64, t19631: f64, t3739: f64, t794: f64, t40018: f64, t6353: f64, t5187: f64) -> (f64, f64, f64) {
    let t56482 = t3739 * t118 * t794 * t19631;
    let t56484 = t40018 * t6353;
    let t56486 = t5187 * t5187;
    (t56482, t56484, t56486)
}
