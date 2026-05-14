//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 721/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk721<F: Float>(t2677: F, t9320: F, t9307: F, t140: F, t190: F, t937: F, t975: F, t1001: F, t981: F) -> (F, F, F, F, F) {
    let t9321 = t2677 * t9320;
    let t9323 = t2677 * t9307;
    let t9326 = t140 * t937 * t190;
    let t9329 = t140 * t975 * t190;
    let t9331 = t981 * t1001;
    (t9321, t9323, t9326, t9329, t9331)
}
