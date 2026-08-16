//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1032/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1032(t3216: f64, t4349: f64, t4344: f64, t997: f64, t1462: f64, t3237: f64, t4764: f64, t12930: f64, t1470: f64, t1423: f64, t3700: f64, t3379: f64, t4410: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17683 = t3216 * t4349;
    let t17687 = t997 * t4344;
    let t17689 = t3237 * t1462;
    let t17691 = t997 * t4764;
    let t17694 = t12930 * t1470;
    let t17701 = t3700 * t1423;
    let t17703 = t3379 * t4410;
    (t17683, t17687, t17689, t17691, t17694, t17701, t17703)
}
