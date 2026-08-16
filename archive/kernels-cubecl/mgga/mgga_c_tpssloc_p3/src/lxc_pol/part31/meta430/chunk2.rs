//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1560/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1560<F: Float>(t22893: F, t6969: F, t22892: F, t3787: F, t6604: F) -> (F, F, F) {
    let t22894 = t22893 * t6969;
    let t22895 = t22892 * t22894;
    let t22896 = F::cast_from(0.16449340668482264365e-1_f64) * t22895;
    let t22897 = t6604 * t3787;
    (t22894, t22896, t22897)
}
