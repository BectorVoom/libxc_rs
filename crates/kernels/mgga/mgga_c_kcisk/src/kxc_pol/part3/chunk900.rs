//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 900/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk900<F: Float>(t1340: F, t13424: F, t1411: F, t3512: F, t3760: F, t3759: F, t12925: F, t1341: F, t1339: F, t1218: F, t338: F, t1327: F, t3922: F) -> (F, F, F, F, F) {
    let t13425 = t1340 * t13424;
    let t13426 = t1411 * t13425;
    let t13428 = t3512 * t3760;
    let t13429 = t3759 * t13428;
    let t13431 = t1341 * t12925;
    let t13432 = t1340 * t13431;
    let t13433 = t1339 * t13432;
    let t13435 = t1218 * t1218;
    let t13436 = F::new(1.0) / t13435;
    let t13437 = t338 * t13436;
    let t13438 = t3922 * t1327;
    (t13426, t13429, t13433, t13437, t13438)
}
