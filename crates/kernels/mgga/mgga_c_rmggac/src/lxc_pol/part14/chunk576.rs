//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 576/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk576<F: Float>(t352: F, t7567: F, t1356: F, t665: F, t833: F, t739: F, t2024: F, t866: F, t36: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t7568 = t7567 * t352;
    let t7569 = t1356 * t7568;
    let t7570 = F::new(0.79828278012425390428e-1) * t7569;
    let t7571 = t665 * t833;
    let t7572 = t739 * t7571;
    let t7573 = F::new(0.59871208509319042821e-1) * t7572;
    let t7574 = t2024 * t866;
    let t7575 = t1356 * t7574;
    let t7576 = F::new(0.39914139006212695214e-1) * t7575;
    let t7577 = t874 * t36;
    (t7568, t7570, t7571, t7573, t7574, t7576, t7577)
}
