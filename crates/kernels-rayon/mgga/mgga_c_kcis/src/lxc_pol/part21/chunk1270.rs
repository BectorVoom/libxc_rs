//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1270/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1270(t14686: f64, t26930: f64, t14718: f64, t28029: f64, t3432: f64, t5026: f64, t3463: f64, t376: f64, t14629: f64, t3178: f64, t5068: f64, t389: f64, t42385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95457 = t26930 * t14686;
    let t95459 = t28029 * t14718;
    let t95461 = t5026 * t3432;
    let t95463 = t3463 * t376;
    let t95464 = t95463 * t14629;
    let t95466 = t3178 * t5068;
    let t95468 = t42385 * t389;
    (t95457, t95459, t95461, t95464, t95466, t95468)
}
