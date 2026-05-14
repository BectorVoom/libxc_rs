//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 802/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk802<F: Float>(t39555: F, t1562: F, t7894: F, t2412: F, t7424: F, t7421: F, t36639: F, t8636: F, t4968: F, t511: F, t2344: F, t2868: F, t7578: F, t623: F, t7191: F, t7194: F) -> (F, F, F, F, F, F, F, F) {
    let t39556 = 0.10248087766267884742e-3 * t39555;
    let t39558 = 0.4726e1 * t1562 * t7894;
    let t39559 = t2412 * t7424;
    let t39561 = t2412 * t7421;
    let t39563 = t36639 * t8636;
    let t39565 = t4968 * t511;
    let t39566 = t39565 * t2344;
    let t39568 = t2868 * t7578;
    let t39570 = t623 * t7191;
    let t39571 = t39570 * t7194;
    (t39556, t39558, t39559, t39561, t39563, t39566, t39568, t39571)
}
