//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 786/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk786<F: Float>(t297: F, t9895: F, t1008: F, t195: F, t1053: F, t3187: F, t1006: F, t3185: F, t3274: F, t213: F, t220: F, t142: F, t79: F, t139: F, t172: F, t3281: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9896 = t297 * t9895;
    let t10334 = t1008 * t1008;
    let t10335 = 1.0 / t10334;
    let t10336 = t195 * t10335;
    let t10337 = t3187 * t1053;
    let t10340 = t1006 * t3185;
    let t10349 = t1053 * t3274;
    let t10447 = t220 * t213;
    let t10471 = t142 * t79;
    let t10500 = t139 * t172 * t79;
    let t10520 = 6.0 * t3281;
    (t9896, t10334, t10335, t10336, t10337, t10340, t10349, t10447, t10471, t10500, t10520)
}
