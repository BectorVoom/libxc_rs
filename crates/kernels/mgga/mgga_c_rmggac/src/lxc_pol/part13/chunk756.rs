//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 756/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk756<F: Float>(t1591: F, t2046: F, t2050: F, t31: F, t1657: F, t638: F, t7292: F, t8486: F, t7498: F, t8659: F, t7505: F, t8365: F, t1971: F, t2144: F, t495: F, t5898: F, t7230: F) -> (F, F, F, F, F, F) {
    let t38881 = t2046 * t2050 * t1591 * t31;
    let t38886 = t2046 * t2050 * t1657 * t31;
    let t38889 = t638 * t7292 * t8486;
    let t38899 = t8659 * t7498;
    let t38901 = t8365 * t7505;
    let t38908 = t7230 * t1971 * t2144 * t5898 * t495;
    (t38881, t38886, t38889, t38899, t38901, t38908)
}
