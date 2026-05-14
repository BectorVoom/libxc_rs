//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 727/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk727<F: Float>(t7487: F, t8466: F, t35207: F, t8469: F, t1591: F, t2046: F, t2050: F, t31: F, t1657: F, t638: F, t7292: F, t8486: F, t7244: F, t9159: F, t1607: F, t1986: F) -> (F, F, F, F, F, F, F) {
    let t38874 = t7487 * t8466;
    let t38876 = t35207 * t8469;
    let t38881 = t2046 * t2050 * t1591 * t31;
    let t38886 = t2046 * t2050 * t1657 * t31;
    let t38889 = t638 * t7292 * t8486;
    let t38934 = t7244 * t9159;
    let t38943 = t1986 * t1607;
    (t38874, t38876, t38881, t38886, t38889, t38934, t38943)
}
