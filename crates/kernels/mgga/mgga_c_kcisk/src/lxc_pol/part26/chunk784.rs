//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 784/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk784<F: Float>(t9834: F, t9837: F, t9840: F, t9842: F, t9844: F, t9846: F) -> (F,) {
    let t9891 = 0.9375e-1 * t9834 - 0.9375e-1 * t9837 + 0.625e-1 * t9840 - 0.20234375e-1 * t9842 + 0.20234375e-1 * t9844 - 0.26979166666666666667e-1 * t9846;
    (t9891,)
}
