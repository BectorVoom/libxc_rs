//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 629/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk629<F: Float>(t1444: F, t538: F, t1466: F, t1527: F, t4121: F, t569: F, t1532: F, t492: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t4230 = t538 * t1444;
    let t4248 = t1527 * t1466;
    let t4249 = t4248 * sigma2;
    let t4254 = t569 * t4121;
    let t4255 = t4254 * sigma2;
    let t4260 = t1532 * t492;
    (t4230, t4248, t4249, t4254, t4255, t4260)
}
