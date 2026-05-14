//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1126/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1126<F: Float>(t10309: F, t6514: F, t10121: F, t7832: F, t10092: F, t2970: F, t6523: F, t3187: F, t1227: F, t1245: F, t2363: F, t2393: F, t3880: F, t410: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10310 = t6514 * t10309;
    let t10311 = t7832 * t10121;
    let t10316 = t2970 * t10092;
    let t10319 = t6523 * t10309;
    let t10320 = t7832 * t3187;
    let t10323 = t1245 * t1227;
    let t10324 = t2363 * t10323;
    let t10331 = t2393 * t10323;
    let t10334 = t410 * t3880;
    (t10310, t10311, t10316, t10319, t10320, t10323, t10324, t10331, t10334)
}
