//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 604/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk604<F: Float>(t254: F, t3344: F, t2333: F, t795: F, t321: F, t502: F, t263: F, t818: F) -> (F, F, F, F) {
    let t3345 = t254 * t3344;
    let t3352 = t2333 * t795;
    let t3356 = t502 * t321;
    let t3358 = t263 * t818;
    (t3345, t3352, t3356, t3358)
}
