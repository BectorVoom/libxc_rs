//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 336/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk336(t1663: f64, t1664: f64, t1645: f64, t1634: f64, t1638: f64) -> (f64, f64, f64) {
    let t1665 = t1663 * t1664;
    let t1667 = 1.0_f64 * t1645 * t1665;
    let t1668 = 0.92708333333333333333e-2_f64 * t1634;
    let t1670 = -t1668 - 0.92708333333333333333e-2_f64 * t1638;
    (t1665, t1667, t1670)
}
