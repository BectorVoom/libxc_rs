//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 572/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk572<F: Float>(t2333: F, t7508: F, t1652: F, t649: F, t27: F, t2145: F, t674: F, t8450: F) -> (F, F, F, F) {
    let t8565 = t7508 * t2333;
    let t8567 = t649 * t1652;
    let t8568 = t27 * t8567;
    let t8569 = t2145 * t8568;
    let t8571 = t8450 * t674;
    (t8565, t8568, t8569, t8571)
}
