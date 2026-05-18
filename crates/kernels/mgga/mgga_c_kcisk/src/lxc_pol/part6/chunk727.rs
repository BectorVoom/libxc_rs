//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 727/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk727<F: Float>(t12825: F, t458: F, t12829: F, t459: F, t12951: F, t13009: F, t420: F, t12974: F, t1390: F, t382: F, t1412: F, t453: F) -> (F, F, F, F, F, F, F, F) {
    let t13220 = t12825 * t458;
    let t13221 = t459 * t12829;
    let t13233 = t459 * t12951;
    let t13244 = t13009 * t420;
    let t13263 = F::new(0.12841111111111111111e-1) * t12974;
    let t13293 = t382 * t1390;
    let t13327 = t1412 * t1412;
    let t13328 = F::new(1.0) / t13327;
    let t13329 = t453 * t13328;
    (t13220, t13221, t13233, t13244, t13263, t13293, t13328, t13329)
}
