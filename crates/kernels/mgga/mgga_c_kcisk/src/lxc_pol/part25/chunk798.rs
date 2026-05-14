//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 798/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk798<F: Float>(t4730: F, t827: F, t1660: F, t2877: F, t4733: F, t4727: F, t5005: F, t79: F, t10568: F, t311: F, t3841: F, t579: F, t571: F, t574: F, t581: F, t4786: F, t596: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10607 = t827 * t4730;
    let t10615 = t2877 * t1660;
    let t10617 = t827 * t4733;
    let t10619 = t827 * t4727;
    let t10621 = t79 * t5005;
    let t10639 = 0.93932222222222222223e0 * t10568;
    let t10641 = t311 * t3841 * t579;
    let t10642 = 0.36793333333333333333e0 * t10641;
    let t10649 = 28.0 / 27.0 * t10568;
    let t10663 = 1.0/pow_3_2(t571);
    let t10671 = 1.0 / t574 / t581 / 4.0;
    let t10690 = 1.0 / t4786 / t596;
    (t10607, t10615, t10617, t10619, t10621, t10639, t10641, t10642, t10649, t10663, t10671, t10690)
}
