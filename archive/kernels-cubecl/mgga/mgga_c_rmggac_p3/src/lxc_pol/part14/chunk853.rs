//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 853/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk853<F: Float>(t1971: F, t2144: F, t495: F, t5267: F, t7230: F, t3351: F, t498: F, t7231: F, t3352: F, t5145: F, t5268: F, t7262: F) -> (F, F, F, F) {
    let t38913 = t7230 * t1971 * t2144 * t5267 * t495;
    let t38918 = t3351 * t7231 * t2144 * t5267 * t498;
    let t38922 = t3351 * t3352 * t2144 * t5145;
    let t38926 = t3351 * t1971 * t7262 * t5268;
    (t38913, t38918, t38922, t38926)
}
