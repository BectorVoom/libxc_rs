//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 335/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk335(t1636: f64, t1659: f64, t26: f64, t1638: f64, t1649: f64, t1651: f64, t1654: f64, t1658: f64, t586: f64) -> (f64, f64, f64, f64) {
    let t1660 = t1659 * t1636;
    let t1661 = t26 * t1660;
    let t1663 = 0.1898925e1_f64 * t1649 - t1651 - 0.29896666666666666667e0_f64 * t1638 + 0.3071625e0_f64 * t1654 - t1658 - 0.82156666666666666667e-1_f64 * t1661;
    let t1664 = 1.0_f64 / t586;
    (t1660, t1661, t1663, t1664)
}
