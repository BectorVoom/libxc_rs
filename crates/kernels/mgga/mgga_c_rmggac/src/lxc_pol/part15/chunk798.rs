//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 798/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk798<F: Float>(t38872: F, t7487: F, t8466: F, t35207: F, t8469: F, t1591: F, t2046: F, t2050: F, t31: F, t1657: F, t638: F, t7292: F, t8486: F) -> (F, F, F, F, F, F) {
    let t38873 = F::new(0.10248087766267884742e-3) * t38872;
    let t38874 = t7487 * t8466;
    let t38876 = t35207 * t8469;
    let t38881 = t2046 * t2050 * t1591 * t31;
    let t38882 = F::new(0.43368970657079495312e-4) * t38881;
    let t38886 = t2046 * t2050 * t1657 * t31;
    let t38887 = F::new(0.43368970657079495312e-4) * t38886;
    let t38889 = t638 * t7292 * t8486;
    (t38873, t38874, t38876, t38882, t38887, t38889)
}
