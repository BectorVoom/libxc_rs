//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 872/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk872<F: Float>(t1341: F, t30238: F, t1340: F, t1339: F, t2177: F, t25308: F, t5606: F, t8089: F, t2075: F, t8240: F, t14265: F, t3482: F, t8255: F, t3484: F, t8077: F, t13377: F) -> (F, F, F, F, F, F) {
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
    let t30256 = t8255 * t2075;
    let t30257 = t3484 * t30256;
    let t30258 = t3482 * t30257;
    let t30260 = t8077 * t2075;
    let t30261 = t13377 * t30260;
    (t30241, t30244, t30247, t30254, t30258, t30261)
}
