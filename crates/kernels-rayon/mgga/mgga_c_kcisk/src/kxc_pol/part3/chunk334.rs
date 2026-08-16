//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 334/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk334(t1646: f64, t1648: f64, t1634: f64, t571: f64, t311: f64, t436: f64, t579: f64, t657: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1649 = t1646 * t1648;
    let t1651 = 0.29896666666666666667e0_f64 * t1634;
    let t1653 = f64::sqrt(t571);
    let t1654 = t1653 * t1648;
    let t1657 = t311 * t436 * t579;
    let t1658 = 0.82156666666666666667e-1_f64 * t1657;
    let t1659 = t79 * t657;
    (t1649, t1651, t1653, t1654, t1657, t1658, t1659)
}
