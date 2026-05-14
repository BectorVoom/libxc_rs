//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1382/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1382<F: Float>(t2721: F, t382: F, t19814: F, t3482: F, t19818: F, t5633: F, t1339: F, t20909: F, t9461: F, t114304: F, t9446: F, t113639: F, t20880: F, t2722: F, t415: F, t109420: F, t1411: F, t33508: F) -> (F, F, F, F, F, F, F) {
    let t114480 = t2721 * t382;
    let t114482 = t3482 * t114480 * t19814;
    let t114487 = t5633 * t114480 * t19818;
    let t114490 = t1339 * t9461 * t20909;
    let t114493 = 0.13888888888888888889e-1 * t9446 * t114304;
    let t114499 = 0.69444444444444444446e-2 * t9446 * t113639;
    let t114505 = t415 * t20880 * t2722;
    let t114510 = t1411 * t109420 * t33508;
    (t114482, t114487, t114490, t114493, t114499, t114505, t114510)
}
