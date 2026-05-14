//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1269/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1269<F: Float>(t12261: F, t9727: F, t2804: F, t33284: F, t9736: F, t33276: F, t9721: F, t112551: F, t33167: F, t33208: F, t33297: F, t9725: F, t33290: F, t9739: F, t33207: F, t9720: F) -> (F, F, F, F, F, F, F, F, F) {
    let t113037 = t12261 * t9727;
    let t113038 = t2804 * t113037;
    let t113040 = t33284 * t9736;
    let t113042 = t9721 * t33276;
    let t113058 = 0.51588271604938271604e-3 * t112551;
    let t113059 = t33208 * t33167;
    let t113061 = t33297 * t33167;
    let t113069 = t9725 * t113037;
    let t113082 = t33290 * t9739;
    let t113085 = t9720 * t33207;
    (t113038, t113040, t113042, t113058, t113059, t113061, t113069, t113082, t113085)
}
