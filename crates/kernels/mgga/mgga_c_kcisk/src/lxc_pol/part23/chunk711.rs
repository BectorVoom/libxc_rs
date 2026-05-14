//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 711/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk711<F: Float>(t6316: F, t6318: F, t4229: F, t491: F, t4304: F, t79: F, t6006: F) -> (F, F, F, F) {
    let t6319 = t6316 * t6318;
    let t6321 = t491 * t4229;
    let t6322 = t79 * t4304;
    let t6323 = t6322 * t6006;
    (t6319, t6321, t6322, t6323)
}
