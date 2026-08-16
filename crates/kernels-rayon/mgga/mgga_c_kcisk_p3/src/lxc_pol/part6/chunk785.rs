//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 785/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk785(t164: f64, t2169: f64, t1309: f64, t2159: f64, t3934: f64, t394: f64, t1224: f64, t13524: f64, t2075: f64) -> (f64, f64, f64) {
    let t20184 = t164 * t2169;
    let t20185 = t1309 * t20184;
    let t20255 = t2159 * t394 * t3934;
    let t20292 = t1224 * t13524 * t2075;
    (t20185, t20255, t20292)
}
