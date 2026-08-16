//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 498/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk498(t1629: f64, t187: f64, t2017: f64, t2070: f64, t2118: f64, t2128: f64, t633: f64, t160: f64, t62: f64, t209: f64, t9: f64, t119: f64, t32: f64, t5: f64) -> (f64, f64, f64, f64) {
    let t2132 = t2017 - t2070 + t187 * (-t1629 * t2128 + t2118 * t633 - t2017 + t2070);
    let t2150 = t62 * t160;
    let t2194 = t209 * t9;
    let t2302 = 0.14764770444444444444e-2_f64 * t5 * t119 * t32;
    (t2132, t2150, t2194, t2302)
}
