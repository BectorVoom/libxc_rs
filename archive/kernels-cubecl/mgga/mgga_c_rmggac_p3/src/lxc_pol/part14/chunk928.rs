//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 928/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk928<F: Float>(t30900: F, t35972: F, t739: F, t36292: F, t5888: F, t118: F, t2001: F, t2281: F, t495: F, t7717: F, t2144: F, t3351: F, t352: F, t7231: F, t9104: F) -> (F, F, F, F) {
    let t39994 = t739 * t35972 * t30900;
    let t39997 = t739 * t36292 * t5888;
    let t39998 = F::cast_from(0.15965655602485078085e0_f64) * t39997;
    let t40001 = t2001 * t118 * t2281 * t495;
    let t40002 = t7717 * t40001;
    let t40007 = t3351 * t7231 * t2144 * t9104 * t352;
    (t39994, t39998, t40002, t40007)
}
