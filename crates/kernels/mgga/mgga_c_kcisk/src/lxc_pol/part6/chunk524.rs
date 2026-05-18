//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 524/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk524<F: Float>(t1203: F, t2097: F, t2105: F, t3725: F, t240: F, t2331: F, t4350: F, t1610: F, t2339: F, t1849: F, t719: F, t4594: F, t704: F) -> (F, F, F, F, F, F, F) {
    let t6549 = t2097 * t1203;
    let t6560 = t2105 * t3725;
    let t6568 = t240 * t2097;
    let t6591 = t2331 * t4350;
    let t6604 = t2339 * t1610;
    let t6666 = t719 * t1849;
    let t6672 = t4594 * t704;
    (t6549, t6560, t6568, t6591, t6604, t6666, t6672)
}
