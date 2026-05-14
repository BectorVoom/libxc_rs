//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 685/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk685<F: Float>(t338: F, t3960: F, t8048: F, t1310: F, t7828: F) -> (F, F, F) {
    let t400 = 0.0 < t338;
    let t8049 = t3960 * t8048;
    let t8050 = t1310 * t8049;
    let t8054 = piecewise3(t400, t7828, -t7828);
    (t8049, t8050, t8054)
}
