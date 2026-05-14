//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 302/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk302<F: Float>(t1910: F, t196: F, t1019: F, t1027: F, t1044: F, t1050: F, t1087: F, t1094: F, t1104: F, t1112: F, t1133: F, t1140: F, t1143: F, t1839: F, t1843: F, t1844: F, t500: F) -> (F, F) {
    let t1911 = t196 * t1910;
    let t1914 = -t1019 - t1027 - t1044 - t1050 + t1133 + 0.186546e0 * t1143 * t1839 - t1094 + t1104 + t1112 - t1087 + t1140 + t1843 + 0.31091e-1 * t1911 * t500 - t1844;
    (t1911, t1914)
}
