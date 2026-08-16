//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 778/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk778(t12647: f64, t639: f64, t1017: f64, t11110: f64, t1885: f64, t587: f64, t1046: f64, t3493: f64, t10686: f64, t3535: f64, t7130: f64, t10908: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12649 = 4.0_f64 / 5.0_f64 * t639 * t12647;
    let t12650 = t11110 * t1017;
    let t12651 = t1885 * t12650;
    let t12653 = 4.0_f64 / 5.0_f64 * t587 * t12651;
    let t12655 = 4.0_f64 / 5.0_f64 * t3493 * t1046;
    let t12656 = 16.0_f64 / 45.0_f64 * t10686;
    let t12658 = 8.0_f64 / 5.0_f64 * t7130 * t3535;
    let t12659 = t10908 * t995;
    (t12649, t12650, t12651, t12653, t12655, t12656, t12658, t12659)
}
