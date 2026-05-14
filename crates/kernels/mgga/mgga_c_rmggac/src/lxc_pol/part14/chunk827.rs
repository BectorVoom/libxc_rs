//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 827/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk827<F: Float>(t40031: F, t7720: F, t495: F, t511: F, t7230: F, t7231: F, t9104: F, t7508: F, t8568: F, t27102: F, t3351: F, t3352: F, t875: F, t2286: F, t34881: F, t7424: F, t8571: F) -> (F, F, F, F, F, F) {
    let t40032 = t7720 * t40031;
    let t40037 = t7230 * t7231 * t511 * t9104 * t495;
    let t40039 = t7508 * t8568;
    let t40043 = t3351 * t3352 * t875 * t27102;
    let t40045 = t34881 * t2286;
    let t40047 = t8571 * t7424;
    (t40032, t40037, t40039, t40043, t40045, t40047)
}
