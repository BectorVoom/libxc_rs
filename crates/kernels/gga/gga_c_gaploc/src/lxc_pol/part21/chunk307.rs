//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 307/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk307<F: Float>(t1328: F, t169: F, t172: F, t452: F, t122: F, t594: F, t599: F) -> (F, F, F, F) {
    let t1329 = t1328 * t169;
    let t1331 = t452 * t1329 * t172;
    let t1338 = t122 * t594;
    let t1339 = t169 * t599;
    (t1329, t1331, t1338, t1339)
}
