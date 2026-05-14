//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 618/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk618<F: Float>(t262: F, t8712: F, t2118: F, t2100: F, t8709: F, t2103: F, t8701: F, t8705: F, t3826: F, t8625: F, t3810: F, t8631: F, t797: F, t8700: F, t2347: F, t6444: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8713 = t262 * t8712;
    let t8714 = t2118 * t8713;
    let t8716 = t2100 * t8709;
    let t8718 = t2103 * t8713;
    let t8720 = t2118 * t8701;
    let t8722 = t2100 * t8705;
    let t8724 = t3826 * t8625;
    let t8726 = t3810 * t8631;
    let t8729 = t797 * t8700;
    let t8731 = t6444 * t2347;
    (t8713, t8714, t8716, t8718, t8720, t8722, t8724, t8726, t8729, t8731)
}
