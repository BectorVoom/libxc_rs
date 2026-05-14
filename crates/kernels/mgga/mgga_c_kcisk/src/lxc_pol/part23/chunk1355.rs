//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1355/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1355<F: Float>(t1339: F, t19765: F, t9461: F, t1322: F, t32069: F, t6204: F, t6217: F, t1327: F, t32033: F, t19777: F, t19077: F, t32045: F, t113745: F, t32008: F, t13485: F, t32087: F, t33423: F) -> (F, F, F, F, F, F, F) {
    let t113779 = t1339 * t9461 * t19765;
    let t113783 = t6204 * t32069 * t6217 * t1322;
    let t113788 = t6204 * t32033 * t6217 * t1327;
    let t113792 = t1339 * t9461 * t19777;
    let t113796 = t1339 * t32045 * t19077;
    let t113800 = t32008 * t113745;
    let t113805 = t32087 * t13485 * t33423;
    (t113779, t113783, t113788, t113792, t113796, t113800, t113805)
}
