//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 982/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk982(t2075: f64, t8255: f64, t3484: f64, t3482: f64, t8077: f64, t13377: f64, t19055: f64, t8172: f64, t8177: f64, t2168: f64, t7740: f64, t3937: f64) -> (f64, f64, f64, f64, f64) {
    let t30256 = t8255 * t2075;
    let t30257 = t3484 * t30256;
    let t30258 = t3482 * t30257;
    let t30260 = t8077 * t2075;
    let t30261 = t13377 * t30260;
    let t30262 = t3482 * t30261;
    let t30264 = t19055 * t8172;
    let t30266 = t19055 * t8177;
    let t30269 = t7740 * t2168;
    let t30270 = t3937 * t30269;
    (t30258, t30262, t30264, t30266, t30270)
}
