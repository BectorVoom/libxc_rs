//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1052/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1052(t2279: f64, t8260: f64, t31141: f64, t6332: f64, t6331: f64, t6388: f64, t8248: f64, t2259: f64, t8256: f64, t6382: f64, t8275: f64, t30489: f64, t4143: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31245 = t8260 * t2279;
    let t31247 = t6332 * t31141;
    let t31248 = t6331 * t31247;
    let t31250 = t6388 * t8248;
    let t31252 = t2259 * t8256;
    let t31254 = t6382 * t8275;
    let t31256 = t4143 * t30489;
    (t31245, t31248, t31250, t31252, t31254, t31256)
}
