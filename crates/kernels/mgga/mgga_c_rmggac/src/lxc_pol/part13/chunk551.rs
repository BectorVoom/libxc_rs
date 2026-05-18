//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 551/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk551<F: Float>(t118: F, t7419: F, t7418: F, t675: F, t1253: F, t1986: F, t211: F, t483: F, t1965: F) -> (F, F, F, F, F, F) {
    let t7420 = t118 * t7419;
    let t7421 = t7418 * t7420;
    let t7422 = t675 * t7421;
    let t7424 = t1986 * t1253;
    let t7425 = t675 * t7424;
    let t7427 = t211 * t483;
    let t7428 = t1965 * t7427;
    (t7421, t7422, t7424, t7425, t7427, t7428)
}
