//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1706/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1706(t12365: f64, t1354: f64, t1307: f64, t3792: f64, t3788: f64, t835: f64, t1336: f64, t3795: f64, t3799: f64, t3853: f64, t3858: f64, t12267: f64, t1340: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12366 = t12365 * t1354;
    let t12369 = t3792 * t1307;
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12386 = t12385 * t3795;
    let t12388 = t3799 * t3853;
    let t12395 = t3799 * t3858;
    let t12397 = t12267 * t1340;
    (t12366, t12369, t12384, t12385, t12386, t12388, t12395, t12397)
}
