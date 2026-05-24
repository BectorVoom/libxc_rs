//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 996/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk996<F: Float>(t30421: F, t30434: F, t1266: F, t1275: F, t1254: F, t30318: F, t2129: F, t7959: F, t2128: F, t26344: F, t2141: F, t4129: F) -> (F, F, F, F, F) {
    let t30435 = t30421 + t30434;
    let t30437 = t1266 * t30435 * t1275;
    let t30442 = t30318 * t1254;
    let t30445 = t2129 * t7959;
    let t30448 = t26344 * t2128;
    let t30451 = t4129 * t2141;
    (t30437, t30442, t30445, t30448, t30451)
}
