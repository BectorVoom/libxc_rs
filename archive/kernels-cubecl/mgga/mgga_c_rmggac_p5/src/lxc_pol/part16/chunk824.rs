//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 824/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk824<F: Float>(t118: F, t2001: F, t498: F, t571: F, t1618: F, t1986: F, t1600: F, t7487: F, t8352: F, t534: F, t7350: F, t7349: F, t7353: F) -> (F, F, F, F, F) {
    let t40699 = t2001 * t118 * t571 * t498;
    let t40702 = t1986 * t1618;
    let t40705 = t1986 * t1600;
    let t40715 = t7487 * t8352;
    let t40717 = t7350 * t534;
    let t40719 = t7349 * t40717 * t7353;
    (t40699, t40702, t40705, t40715, t40719)
}
