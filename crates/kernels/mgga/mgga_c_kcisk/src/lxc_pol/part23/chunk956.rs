//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 956/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk956<F: Float>(t425: F, t5684: F, t1364: F, t3564: F, t11313: F, t2218: F, t1175: F, t2059: F, t12849: F, t1390: F, t1428: F, t1056: F, t180: F, t3529: F, t479: F, t3532: F) -> (F, F, F, F, F, F, F, F) {
    let t19399 = t425 * t5684;
    let t19400 = t19399 * t1364;
    let t19401 = t3564 * t19400;
    let t19404 = t11313 * t2218;
    let t19407 = t2059 * t1175;
    let t19409 = t12849 * t19407 * t1364;
    let t19412 = t1428 * t1390;
    let t19413 = t19407 * t1056;
    let t19414 = t19412 * t19413;
    let t19418 = t180 * t479 * t3529;
    let t19419 = t1428 * t3532;
    (t19400, t19401, t19404, t19409, t19413, t19414, t19418, t19419)
}
