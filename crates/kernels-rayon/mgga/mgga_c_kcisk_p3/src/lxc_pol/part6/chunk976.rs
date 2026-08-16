//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 976/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk976(t19849: f64, t8073: f64, t1411: f64, t5606: f64, t7832: f64, t10519: f64, t10520: f64, t30158: f64, t8: f64, t1450: f64, t1340: f64, t2075: f64, t8247: f64) -> (f64, f64, f64, f64, f64) {
    let t30197 = t19849 * t8073;
    let t30198 = t1411 * t30197;
    let t30201 = t5606 * t7832;
    let t30202 = t1411 * t30201;
    let t30205 = t30158 * t8 + t10519 + t10520;
    let t30206 = t1450 * t30205;
    let t30207 = t1340 * t30206;
    let t30208 = t1411 * t30207;
    let t30212 = t8247 * t2075;
    (t30198, t30202, t30205, t30208, t30212)
}
