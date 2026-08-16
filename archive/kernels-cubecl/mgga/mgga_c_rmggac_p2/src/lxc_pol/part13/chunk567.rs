//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 567/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk567<F: Float>(t2147: F, t7508: F, t649: F, t866: F, t27: F, t2145: F, t645: F, t798: F, t3928: F, t2060: F, t4048: F, t1550: F) -> (F, F, F, F, F, F, F) {
    let t7509 = t7508 * t2147;
    let t7511 = t649 * t866;
    let t7512 = t27 * t7511;
    let t7513 = t2145 * t7512;
    let t7518 = t645 * t798;
    let t7519 = t3928 * t7518;
    let t7521 = t2060 * t4048;
    let t7522 = t1550 * t7521;
    (t7509, t7512, t7513, t7518, t7519, t7521, t7522)
}
