//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 536/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk536(t4170: f64, t5655: f64, t4160: f64, t3751: f64, t540: f64, t1017: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t5656 = t4170 * t5655;
    let t5657 = t4160 * t5656;
    let t5659 = t3751 * t540;
    let t5661 = t86 * t1017 * t5659;
    (t5656, t5657, t5659, t5661)
}
