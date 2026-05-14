//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 483/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk483<F: Float>(t642: F, t695: F, t1755: F, t654: F, t1906: F, t751: F, t724: F, t574: F, t725: F, t140: F, t430: F, t728: F, t41: F, t4594: F, t4597: F, t702: F) -> (F, F, F, F, F, F, F, F) {
    let t5193 = t642 * t695;
    let t5203 = t654 * t1755;
    let t5217 = 1.0 / t1906 / t751;
    let t5218 = t724 * t5217;
    let t5231 = t725 * t574;
    let t5242 = 0.88437037037037037037e-2 * t140 * t430 * t728;
    let t5248 = t41 * t4594;
    let t5249 = t702 * t4597;
    (t5193, t5203, t5217, t5218, t5231, t5242, t5248, t5249)
}
