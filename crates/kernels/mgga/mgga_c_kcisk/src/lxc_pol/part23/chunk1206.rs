//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1206/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1206<F: Float>(t32000: F, t9814: F, t1339: F, t1322: F, t2173: F, t32069: F, t6204: F) -> (F, F, F, F, F) {
    let t33392 = t32000 * t9814;
    let t33393 = t1339 * t33392;
    let t33398 = t2173 * t1322;
    let t33399 = t32069 * t33398;
    let t33400 = t6204 * t33399;
    (t33392, t33393, t33398, t33399, t33400)
}
