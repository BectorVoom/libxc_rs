//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 337/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk337(t222: f64, t227: f64, t2063: f64, t229: f64, t2062: f64, t44: f64, t2059: f64, t295: f64, t442: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t2066 = piecewise3(t228, 0.0_f64, 4.0_f64 / 3.0_f64 * t229 * t2063);
    let t2068 = (t2062 + t2066) * t44;
    let t2070 = piecewise3(t223, 0.0_f64, t2059);
    let t2071 = t295 * t2070;
    let t2075 = t442 * t2059;
    (t2068, t2071, t2075)
}
