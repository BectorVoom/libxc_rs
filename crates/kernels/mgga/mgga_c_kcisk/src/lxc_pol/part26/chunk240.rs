//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 240/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk240<F: Float>(t1175: F, t1180: F, t311: F, t313: F, t436: F, t398: F, t79: F) -> (F, F, F, F) {
    let t1181 = t1180 * t1175;
    let t1184 = t311 * t436 * t313;
    let t1185 = 0.82156666666666666667e-1 * t1184;
    let t1186 = t79 * t398;
    (t1181, t1184, t1185, t1186)
}
