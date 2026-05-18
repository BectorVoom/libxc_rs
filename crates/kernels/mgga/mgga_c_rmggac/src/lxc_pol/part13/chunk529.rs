//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 529/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk529<F: Float>(t2164: F, t356: F, t638: F, t639: F, t1276: F, t640: F, t1173: F, t205: F, t671: F) -> (F, F, F, F, F, F) {
    let t7220 = t2164 * t356;
    let t7222 = t638 * t639 * t7220;
    let t7224 = t640 * t1276;
    let t7226 = t638 * t639 * t7224;
    let t7228 = t1173 * t205;
    let t7229 = t671 * t7228;
    (t7220, t7222, t7224, t7226, t7228, t7229)
}
