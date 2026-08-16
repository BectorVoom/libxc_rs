//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 525/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk525<F: Float>(t26: F, t495: F, t2067: F, t3369: F, t14230: F, t14229: F, t1966: F) -> (F, F, F) {
    let t14231 = t26 * t495;
    let t14232 = t2067 * t14231;
    let t14233 = t3369 * t14232;
    let t14234 = t14230 * t14233;
    let t14236 = t1966 * t14229;
    (t14233, t14234, t14236)
}
