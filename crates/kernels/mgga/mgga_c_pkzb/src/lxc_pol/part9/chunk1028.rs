//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1028/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1028<F: Float>(t758: F, t8419: F, t3186: F, t6417: F, t406: F, t178: F, t6457: F, t6515: F) -> (F, F, F, F, F) {
    let t8420 = t758 * t8419;
    let t8423 = t3186 * t6417;
    let t8424 = t406 * t8423;
    let t8427 = t6457 * t178;
    let t8428 = t6515 * t8427;
    (t8420, t8423, t8424, t8427, t8428)
}
