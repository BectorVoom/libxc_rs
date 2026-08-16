//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2690/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2690(t12189: f64, t6358: f64, t16081: f64, t19795: f64, t1307: f64, t54718: f64, t56463: f64, t686: f64, t16094: f64, t16095: f64, t5187: f64, t56467: f64) -> (f64, f64, f64, f64, f64) {
    let t56491 = t12189 * t6358;
    let t56493 = t16081 * t19795;
    let t56501 = t54718 * t686 * t56463 * t1307;
    let t56505 = t16094 * t686 * t16095 * t5187;
    let t56514 = t16094 * t686 * t56467 * t1307;
    (t56491, t56493, t56501, t56505, t56514)
}
