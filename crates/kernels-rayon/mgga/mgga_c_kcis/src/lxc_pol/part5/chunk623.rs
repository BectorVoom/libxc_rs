//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 623/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk623(t1398: f64, t4142: f64, t1017: f64, t3751: f64, t86: f64, t1392: f64, t540: f64) -> (f64, f64, f64, f64) {
    let t4143 = t4142 * t1398;
    let t4153 = t86 * t1017 * t3751;
    let t4158 = t1392 * t540;
    let t4160 = t86 * t1017 * t4158;
    (t4143, t4153, t4158, t4160)
}
