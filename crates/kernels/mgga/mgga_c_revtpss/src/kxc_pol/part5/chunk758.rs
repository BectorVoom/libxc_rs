//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 758/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk758<F: Float>(t1715: F, t3634: F, t247: F, t1261: F, t1260: F, t1785: F) -> (F, F, F) {
    let t5377 = t3634 * t1715;
    let t5378 = t247 * t5377;
    let t5379 = t1261 * t5378;
    let t5381 = t1785 * t1260;
    (t5378, t5379, t5381)
}
