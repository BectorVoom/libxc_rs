//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1051/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1051(t31141: f64, t6369: f64, t6368: f64, t31165: f64, t4204: f64, t6331: f64, t31170: f64, t4231: f64, t4230: f64, t4203: f64, t21331: f64, t8271: f64) -> (f64, f64, f64, f64, f64) {
    let t31231 = t6369 * t31141;
    let t31232 = t6368 * t31231;
    let t31234 = t4204 * t31165;
    let t31235 = t6331 * t31234;
    let t31237 = t4231 * t31170;
    let t31238 = t4230 * t31237;
    let t31240 = t4204 * t31170;
    let t31241 = t4203 * t31240;
    let t31243 = t21331 * t8271;
    (t31232, t31235, t31238, t31241, t31243)
}
