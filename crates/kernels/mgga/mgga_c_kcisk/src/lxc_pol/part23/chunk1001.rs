//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1001/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1001<F: Float>(t2159: F, t3934: F, t394: F, t19127: F, t6183: F, t19114: F, t6175: F, t13715: F, t2141: F, t4101: F, t4129: F, t4126: F, t6119: F, t6125: F, t13570: F, t6100: F) -> (F, F, F, F, F, F) {
    let t20255 = t2159 * t394 * t3934;
    let t20258 = t6183 * t19127;
    let t20261 = t6175 * t19114;
    let t20264 = t13715 * t2141;
    let t20265 = t4129 * t4101;
    let t20266 = t20264 * t20265;
    let t20269 = t4126 * t6119;
    let t20270 = t20269 * t6125;
    let t20273 = t6100 * t13570;
    (t20255, t20258, t20261, t20266, t20270, t20273)
}
