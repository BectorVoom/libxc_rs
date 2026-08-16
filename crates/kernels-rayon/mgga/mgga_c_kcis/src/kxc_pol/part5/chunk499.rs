//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 499/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk499(t1629: f64, t187: f64, t2017: f64, t2070: f64, t2118: f64, t2128: f64, t633: f64, t449: f64, t160: f64, t62: f64, t209: f64, t9: f64) -> (f64, f64, f64, f64) {
    let t2132 = t2017 - t2070 + t187 * (-t1629 * t2128 + t2118 * t633 - t2017 + t2070);
    let t2133 = t449 * t2132;
    let t2150 = t62 * t160;
    let t2194 = t209 * t9;
    (t2132, t2133, t2150, t2194)
}
