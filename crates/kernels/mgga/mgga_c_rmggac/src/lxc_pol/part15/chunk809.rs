//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 809/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk809<F: Float>(t39535: F, t5898: F, t7778: F, t903: F, t290: F, t38843: F, t2012: F, t7349: F, t1562: F, t7894: F, t623: F, t7191: F) -> (F, F, F, F, F, F) {
    let t39536 = F::new(0.23948483403727617128e0) * t39535;
    let t39544 = t903 * t7778 * t5898;
    let t39545 = F::new(0.23948483403727617128e0) * t39544;
    let t39553 = t290 * t38843;
    let t39555 = t7349 * t2012 * t39553;
    let t39556 = F::new(0.10248087766267884742e-3) * t39555;
    let t39558 = F::new(0.4726e1) * t1562 * t7894;
    let t39570 = t623 * t7191;
    (t39536, t39545, t39553, t39556, t39558, t39570)
}
