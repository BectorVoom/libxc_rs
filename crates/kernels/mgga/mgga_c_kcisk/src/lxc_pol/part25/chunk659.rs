//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 659/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk659<F: Float>(t1421: F, t4586: F, t4587: F, t4589: F, t4591: F, t5913: F, t6998: F, t7001: F, t7005: F, t7009: F, t7013: F, t7017: F, t7020: F, t2372: F, t695: F, t1060: F, t4604: F) -> (F, F, F) {
    let t7022 = -t4586 + 0.43802864444444444445e-3 * t4587 + 0.98556445e-3 * t4589 - 0.65704296666666666667e-3 * t4591 + 0.43802864444444444445e-3 * t6998 + 0.10950716111111111111e-2 * t1421 * t7001 + 0.98556445e-3 * t1421 * t7005 - 0.65704296666666666667e-3 * t1421 * t7009 - 0.13140859333333333333e-2 * t1421 * t7013 - 0.13140859333333333333e-2 * t5913 * t7017 + 0.98556445e-3 * t7020;
    let t7023 = t2372 * t695;
    let t7025 = t4604 * t7023 * t1060;
    (t7022, t7023, t7025)
}
