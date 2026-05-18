//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 868/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk868<F: Float>(t1981: F, t516: F, t676: F, t8512: F, t49: F, t529: F, t36940: F, t36945: F, t68: F, t2411: F, t678: F, t7920: F) -> (F, F, F, F) {
    let t39112 = t8512 * t1981 * t676 * t516;
    let t39116 = t49 * t529;
    let t39119 = t36945 * t39116 * t68 * t36940;
    let t39122 = t2411 * t7920 * t678;
    (t39112, t39116, t39119, t39122)
}
