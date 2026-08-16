//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 727/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk727<F: Float>(t34709: F, t7558: F, t7349: F, t7359: F, t7760: F, t7352: F, t934: F, t2010: F, t7755: F, t7197: F, t892: F, t7203: F) -> (F, F, F, F, F, F) {
    let t34710 = t34709 * t7558;
    let t34711 = F::cast_from(0.65053455985619242968e-4_f64) * t34710;
    let t34713 = t7349 * t7359 * t7760;
    let t34715 = t934 * t7352;
    let t34717 = t2010 * t7755 * t34715;
    let t34724 = t892 * t7197;
    let t34735 = t892 * t7203;
    (t34711, t34713, t34715, t34717, t34724, t34735)
}
