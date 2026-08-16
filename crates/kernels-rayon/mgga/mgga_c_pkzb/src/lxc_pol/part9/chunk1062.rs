//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1062/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1062(t4885: f64, t501: f64, t1542: f64, t1662: f64, t1497: f64, t1503: f64, t1507: f64, t555: f64, t1511: f64, t5146: f64, t4871: f64, t4874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16584 = t501 * t4885;
    let t16586 = t1542 * t1662;
    let t16588 = t1497 * t1497;
    let t16592 = 0.51947577317044391277e2_f64 * t555 * t1503 * t16588 * t1507;
    let t16593 = t1511 * t5146;
    let t16595 = t4871 * t4874;
    (t16584, t16586, t16588, t16592, t16593, t16595)
}
