//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 831/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk831<F: Float>(t16156: F, t8508: F, t3351: F, t511: F, t5226: F, t9188: F, t8808: F, t8504: F, t1001: F, t558: F, t9210: F, t16043: F, t9184: F) -> (F, F, F, F, F, F) {
    let t38704 = t16156 * t8508;
    let t38708 = t3351 * t9188 * t511 * t5226;
    let t38710 = t16156 * t8808;
    let t38712 = t16156 * t8504;
    let t38717 = t3351 * t9210 * t511 * t558 * t1001;
    let t38719 = t16043 * t9184;
    (t38704, t38708, t38710, t38712, t38717, t38719)
}
