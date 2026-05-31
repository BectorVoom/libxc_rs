//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 891/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk891<F: Float>(t13327: F, t453: F, t1440: F, t3786: F, t1341: F, t1411: F, t3764: F, t3785: F, t12957: F, t1340: F, t1339: F, t3748: F, t3770: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t13328 = F::cast_from(1.0_f64) / t13327;
    let t13329 = t453 * t13328;
    let t13330 = t13329 * sigma0;
    let t13331 = t3786 * t1440;
    let t13332 = t1341 * t13331;
    let t13333 = t13330 * t13332;
    let t13334 = t1411 * t13333;
    let t13336 = t3764 * t3786;
    let t13337 = t3785 * t13336;
    let t13338 = t1411 * t13337;
    let t13340 = t1341 * t12957;
    let t13341 = t1340 * t13340;
    let t13342 = t1339 * t13341;
    let t13344 = t3748 * t3770;
    (t13328, t13329, t13331, t13334, t13338, t13342, t13344)
}
