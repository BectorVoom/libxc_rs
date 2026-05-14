//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1181/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1181<F: Float>(t34177: F, t415: F, t1790: F, t2464: F, t32935: F, t7261: F) -> (F, F, F, F) {
    let t34178 = t415 * t34177;
    let t34180 = t2464 * t1790;
    let t34181 = t32935 * t34180;
    let t34182 = t7261 * t34181;
    (t34178, t34180, t34181, t34182)
}
