//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1647/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1647(t1307: f64, t26421: f64, t26446: f64, t26331: f64, t16036: f64, t550: f64, t6976: f64, t1992: f64, t16040: f64, t7696: f64, t794: f64, t6897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26447 = t26421 * t1307;
    let t26448 = t26446 * t26447;
    let t26449 = t26331 * t26448;
    let t26461 = t16036 * t550;
    let t26462 = t6976 * t26461;
    let t26463 = t1992 * t26462;
    let t26466 = t16040 * t550;
    let t26467 = t6976 * t26466;
    let t26468 = t1992 * t26467;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    (t26447, t26448, t26449, t26461, t26462, t26463, t26466, t26467, t26468, t26474, t26475)
}
