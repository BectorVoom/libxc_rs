//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 390/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk390<F: Float>(t2509: F, t720: F, t415: F, t1876: F, t1877: F, t2063: F, t1882: F, t2372: F) -> (F, F, F, F) {
    let t2510 = t2509 * t720;
    let t2511 = t415 * t2510;
    let t2514 = t1876 * t1877 * t2063;
    let t2517 = t1882 * t2372;
    (t2510, t2511, t2514, t2517)
}
