//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1084/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1084<F: Float>(t9422: F, t9442: F, t1405: F, t1413: F, t1292: F, t3930: F, t1308: F) -> (F, F, F, F) {
    let t32055 = t9422 * t9442;
    let t32058 = t1405 * t1413;
    let t32065 = t3930 * t1292;
    let t32066 = t32065 * t1308;
    (t32055, t32058, t32065, t32066)
}
