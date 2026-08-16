//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 572/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk572(t3069: f64, t852: f64, t833: f64, t1184: f64, t2242: f64, t851: f64, t2240: f64, t2175: f64, t2246: f64, t3017: f64, t3028: f64, t1189: f64, t862: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3070 = t3069 * t852;
    let t3072 = 1.0_f64 * t833 * t3070;
    let t3073 = t1184 * t2242;
    let t3074 = t3073 * t851;
    let t3076 = 0.16081979498692535067e2_f64 * t2240 * t3074;
    let t3080 = t2246 - 0.17123333333333333333e-1_f64 * t2175 - 0.17123333333333333333e-1_f64 * t3017 + 0.5137e-1_f64 * t3028;
    let t3083 = t1189 * t862;
    (t3070, t3072, t3073, t3074, t3076, t3080, t3083)
}
