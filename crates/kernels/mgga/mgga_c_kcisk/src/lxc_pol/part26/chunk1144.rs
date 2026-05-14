//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1144/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1144<F: Float>(t33593: F, t9446: F, t1440: F, t2266: F, t32045: F, t1411: F, t1286: F, t2153: F, t9461: F, t2270: F, t394: F) -> (F, F, F, F, F, F, F, F) {
    let t33594 = t9446 * t33593;
    let t33596 = t2266 * t1440;
    let t33597 = t32045 * t33596;
    let t33598 = t1411 * t33597;
    let t33600 = t2153 * t1286;
    let t33601 = t9461 * t33600;
    let t33602 = t1411 * t33601;
    let t33604 = t2270 * t394;
    (t33594, t33596, t33597, t33598, t33600, t33601, t33602, t33604)
}
