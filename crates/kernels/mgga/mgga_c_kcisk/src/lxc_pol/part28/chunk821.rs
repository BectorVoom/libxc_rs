//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 821/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk821<F: Float>(t9970: F, t9973: F, t9975: F, t9978: F, t9980: F, t9982: F, t9984: F, t9986: F) -> (F,) {
    let t10039 = 0.9375e-1 * t9970 - 0.9375e-1 * t9973 - 0.25e0 * t9975 + 0.625e-1 * t9978 - 0.20234375e-1 * t9980 + 0.20234375e-1 * t9982 + 0.10791666666666666667e0 * t9984 - 0.26979166666666666667e-1 * t9986;
    (t10039,)
}
