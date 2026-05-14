//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 166/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk166<F: Float>(t340: F, t639: F, t642: F, rho1: F, sigma2: F) -> (F, F, F, F) {
    let t645 = 10.0 / 9.0 * t340 * t639 * t642;
    let t646 = t645 < -0.66725e-1;
    let t648 = piecewise3(t646, 0.0, 0.66725e-1 + t645);
    let t649 = t648 * sigma2;
    let t650 = rho1 * rho1;
    let t651 = pow_1_3(rho1);
    let t652 = t651 * t651;
    let t654 = 1.0 / t652 / t650;
    (t649, t650, t651, t654)
}
