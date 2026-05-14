//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1132/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1132<F: Float>(t23564: F, t8392: F, t23465: F, t1882: F, t23515: F, t23460: F, t2178: F, t5968: F, t358: F, t23990: F, t23440: F, t23550: F, t23573: F, t23431: F, t5958: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t95471 = t8392 * t23564;
    let t95477 = t8392 * t23465;
    let t95487 = t1882 * t23515;
    let t95492 = t8392 * t23460;
    let t95521 = t2178 * t5968;
    let t95541 = t5968 * t358;
    let t95548 = t1882 * t23990;
    let t95559 = t1882 * t23440;
    let t95573 = t8392 * t23550;
    let t95601 = t8392 * t23573;
    let t95625 = t1882 * t23431;
    let t95632 = t8232 * t5958;
    (t95471, t95477, t95487, t95492, t95521, t95541, t95548, t95559, t95573, t95601, t95625, t95632)
}
