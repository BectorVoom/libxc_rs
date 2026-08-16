//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 981/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk981(t1341: f64, t30238: f64, t1340: f64, t1339: f64, t2177: f64, t25308: f64, t5606: f64, t8089: f64, t2075: f64, t8240: f64, t14265: f64, t3482: f64) -> (f64, f64, f64, f64) {
    let t30239 = t1341 * t30238;
    let t30240 = t1340 * t30239;
    let t30241 = t1339 * t30240;
    let t30243 = t25308 * t2177;
    let t30244 = t1339 * t30243;
    let t30246 = t5606 * t8089;
    let t30247 = t1339 * t30246;
    let t30252 = t8240 * t2075;
    let t30253 = t14265 * t30252;
    let t30254 = t3482 * t30253;
    (t30241, t30244, t30247, t30254)
}
