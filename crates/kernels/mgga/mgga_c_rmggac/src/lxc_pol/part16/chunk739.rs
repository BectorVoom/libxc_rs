//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 739/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk739<F: Float>(t1540: F, t2144: F, t36734: F, t8443: F, t7244: F, t8437: F, t36292: F, t5888: F, t739: F, t118: F, t2001: F, t2281: F, t495: F, t305: F, t321: F, t2286: F, t34881: F) -> (F, F, F, F, F, F, F) {
    let t39953 = t1540 * t2144;
    let t39970 = t36734 * t8443;
    let t39977 = t7244 * t8437;
    let t39997 = t739 * t36292 * t5888;
    let t40001 = t2001 * t118 * t2281 * t495;
    let t40031 = t2001 * t305 * t2281 * t321;
    let t40045 = t34881 * t2286;
    (t39953, t39970, t39977, t39997, t40001, t40031, t40045)
}
