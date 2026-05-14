//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 258/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk258<F: Float>(t1135: F, t2: F, t577: F, t428: F, t1044: F, t1050: F, t1087: F, t1094: F, t1104: F, t1112: F, t1133: F, t1140: F, t1422: F, t1424: F, t1429: F, t1433: F) -> (F, F, F, F, F) {
    let t1434 = 0.18311447306006545054e-3 * t1135;
    let t1435 = t577 * t2;
    let t1436 = t1435 * t428;
    let t1437 = 0.18311447306006545054e-3 * t1436;
    let t1438 = -t1422 - t1044 - t1424 + t1429 - t1050 + t1133 - t1094 + t1104 + t1112 - t1087 - t1434 + t1140 - t1437;
    let t1439 = t1433 + t1438;
    (t1434, t1435, t1436, t1437, t1439)
}
