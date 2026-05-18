//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 945/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk945<F: Float>(t10305: F, t951: F, t3874: F, t410: F, t6514: F, t10121: F, t7832: F, t10092: F, t2970: F, t6523: F, t3187: F, t1227: F, t1245: F) -> (F, F, F, F, F, F, F, F) {
    let t10306 = t10305 * t951;
    let t10309 = t410 * t3874;
    let t10310 = t6514 * t10309;
    let t10311 = t7832 * t10121;
    let t10316 = t2970 * t10092;
    let t10319 = t6523 * t10309;
    let t10320 = t7832 * t3187;
    let t10323 = t1245 * t1227;
    (t10306, t10309, t10310, t10311, t10316, t10319, t10320, t10323)
}
