//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 728/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk728<F: Float>(t817: F, t312: F, t317: F, t6100: F, t832: F, t325: F) -> (F, F, F, F, F, F, F) {
    let t6659 = t817 * t817;
    let t6660 = 1.0 / t6659;
    let t6661 = t312 * t6660;
    let t6678 = 154.0 / 27.0 * t317 * t6100;
    let t6691 = t832 * t832;
    let t6692 = 1.0 / t6691;
    let t6693 = t325 * t6692;
    (t6659, t6660, t6661, t6678, t6691, t6692, t6693)
}
