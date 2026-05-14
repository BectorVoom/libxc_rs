//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 413/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk413<F: Float>(t1002: F, t3132: F, t979: F, t189: F, t980: F, t177: F) -> (F, F, F, F) {
    let t3133 = t3132 * t1002;
    let t3134 = t979 * t3133;
    let t3137 = 1.0 / t980 / t189;
    let t3138 = t177 * t3137;
    (t3133, t3134, t3137, t3138)
}
