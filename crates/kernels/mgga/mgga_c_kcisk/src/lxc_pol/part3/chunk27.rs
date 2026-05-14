//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 27/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk27<F: Float>(t3: F, t41: F) -> (F, F, F, F) {
    let t62 = M_CBRT6;
    let t63 = t62 * t62;
    let t64 = t41 * t3 * t63;
    let t65 = M_PI * M_PI;
    let t66 = pow_1_3(t65);
    let t67 = 1.0 / t66;
    (t63, t64, t65, t67)
}
