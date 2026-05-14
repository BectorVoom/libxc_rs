//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1022/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1022<F: Float>(t24237: F, t24247: F, t24197: F, t1403: F, t24424: F, t681: F, t24181: F, t683: F, t24234: F, t2404: F, t6008: F, t24192: F, t24187: F, t24216: F, t24207: F, t25499: F, t5: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t98159 = t24237 * t24247;
    let t98161 = t24237 * t24197;
    let t98166 = t1403 * t681 * t24424;
    let t98168 = t683 * t24181;
    let t98172 = t24237 * t24234;
    let t98195 = t2404 * t6008;
    let t98208 = t1403 * t681 * t24192;
    let t98211 = t1403 * t681 * t24187;
    let t98214 = t1403 * t681 * t24216;
    let t98219 = t1403 * t681 * t24207;
    let t98250 = t5 * t25499;
    (t98159, t98161, t98166, t98168, t98172, t98195, t98208, t98211, t98214, t98219, t98250)
}
