//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 349/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk349(t1547: f64, t894: f64, t901: f64, t1539: f64, t908: f64, t136: f64, t1541: f64, t899: f64, t907: f64, t913: f64, t893: f64, t917: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1548 = t894 * t1547;
    let t1551 = t901 * t1547;
    let t1553 = t908 * t1539;
    let t1554 = t136 * t1553;
    let t1556 = 0.1898925e1_f64 * t1548 - t899 - 0.29896666666666666667e0_f64 * t1541 + 0.3071625e0_f64 * t1551 - t907 - 0.82156666666666666667e-1_f64 * t1554;
    let t1557 = t1556 * t913;
    let t1559 = 1.0_f64 * t893 * t1557;
    let t1561 = -t917 - 0.17123333333333333333e-1_f64 * t1541;
    (t1548, t1551, t1553, t1554, t1556, t1557, t1559, t1561)
}
