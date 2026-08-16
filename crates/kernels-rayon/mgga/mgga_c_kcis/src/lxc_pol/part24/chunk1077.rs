//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1077/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1077(t28131: f64, t5329: f64, t1267: f64, t1856: f64, t26975: f64) -> (f64, f64, f64, f64) {
    let t28132 = t5329 * t28131;
    let t28135 = t1856 * t1267;
    let t28136 = t26975 * t28135;
    let t28137 = t5329 * t28136;
    (t28132, t28135, t28136, t28137)
}
