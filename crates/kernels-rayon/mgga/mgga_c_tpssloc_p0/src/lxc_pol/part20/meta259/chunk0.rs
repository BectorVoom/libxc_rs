//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1396/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1396(t225: f64, t3023: f64, t1053: f64, t68: f64, t1065: f64, t3175: f64, t3021: f64, t3206: f64, t3174: f64, t1887: f64, t337: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10160 = t3023 * t225;
    let t10163 = t1053 * t1053;
    let t10164 = 1.0_f64 / t10163;
    let t10165 = t68 * t10164;
    let t10166 = t3175 * t1065;
    let t10167 = t10165 * t10166;
    let t10170 = t3021 * t225;
    let t10181 = t1065 * t3206;
    let t10182 = t3174 * t10181;
    let t10186 = t615 * t337 * t1887;
    (t10160, t10163, t10164, t10165, t10166, t10167, t10170, t10181, t10182, t10186)
}
