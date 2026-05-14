//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 583/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk583<F: Float>(t2083: F, t3651: F, t1175: F, t1180: F, t5684: F, t2089: F, t827: F) -> (F, F, F, F) {
    let t5730 = t3651 * t2083;
    let t5731 = t5730 * t1175;
    let t5733 = t1180 * t5684;
    let t5736 = t827 * t2089;
    (t5730, t5731, t5733, t5736)
}
