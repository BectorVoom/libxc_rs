//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 730/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk730<F: Float>(t7335: F, t7766: F, t7334: F, t7552: F, t7558: F, t7349: F, t7359: F, t7760: F, t7352: F, t934: F, t2010: F, t7755: F) -> (F, F, F, F, F) {
    let t34706 = t7335 * t7766;
    let t34709 = t7334 * t7552;
    let t34710 = t34709 * t7558;
    let t34713 = t7349 * t7359 * t7760;
    let t34715 = t934 * t7352;
    let t34717 = t2010 * t7755 * t34715;
    (t34706, t34710, t34713, t34715, t34717)
}
