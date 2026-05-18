//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1342/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1342<F: Float>(t3738: F, t7287: F, t29433: F, t94805: F, t1548: F, t21956: F, t22410: F, t7952: F, t22329: F, t94785: F, t15808: F, t2062: F) -> (F, F, F, F, F, F) {
    let t102963 = t3738 * t7287;
    let t102965 = t94805 * t29433;
    let t102967 = t21956 * t1548;
    let t102969 = t7952 * t22410;
    let t102971 = t94785 * t22329;
    let t102973 = t15808 * t2062;
    (t102963, t102965, t102967, t102969, t102971, t102973)
}
