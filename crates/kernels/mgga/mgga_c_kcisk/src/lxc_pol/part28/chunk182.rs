//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 182/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk182<F: Float>(t819: F, t821: F, t825: F, t827: F, t31: F) -> (F, F) {
    let t829 = -0.632975e0 * t819 - 0.29896666666666666667e0 * t821 - 0.1023875e0 * t825 - 0.82156666666666666667e-1 * t827;
    let t830 = 1.0 / t31;
    (t829, t830)
}
