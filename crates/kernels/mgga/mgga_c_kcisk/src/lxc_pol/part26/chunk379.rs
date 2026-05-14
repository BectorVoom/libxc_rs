//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 379/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk379<F: Float>(t1328: F, t2173: F, t1341: F, t2075: F, t1340: F, t1339: F, t2059: F, t425: F) -> (F, F, F, F, F) {
    let t2174 = t2173 * t1328;
    let t2177 = t1341 * t2075;
    let t2178 = t1340 * t2177;
    let t2179 = t1339 * t2178;
    let t2181 = t425 * t2059;
    (t2174, t2177, t2178, t2179, t2181)
}
