//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1816/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1816(t10599: f64, t1547: f64, t2799: f64, t13615: f64, t894: f64, t1553: f64, t2403: f64) -> (f64, f64, f64, f64) {
    let t13637 = t10599 * t1547;
    let t13638 = t13637 * t2799;
    let t13640 = t894 * t13615;
    let t13642 = t2403 * t1553;
    (t13637, t13638, t13640, t13642)
}
