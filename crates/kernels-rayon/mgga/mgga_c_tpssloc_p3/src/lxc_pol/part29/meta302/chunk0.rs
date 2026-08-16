//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1344/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1344(t225: f64, t3023: f64, t1053: f64, t68: f64, t3021: f64, t1887: f64, t337: f64, t615: f64, t134: f64, t976: f64, t984: f64, t2990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10160 = t3023 * t225;
    let t10163 = t1053 * t1053;
    let t10164 = 1.0_f64 / t10163;
    let t10165 = t68 * t10164;
    let t10170 = t3021 * t225;
    let t10186 = t615 * t337 * t1887;
    let t10189 = t134 * t976;
    let t10190 = t10189 * t984;
    let t10191 = t10190 * t2990;
    (t10160, t10165, t10170, t10186, t10189, t10190, t10191)
}
