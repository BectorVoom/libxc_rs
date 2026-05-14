//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 848/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk848<F: Float>(t2144: F, t3351: F, t3352: F, t6583: F, t1971: F, t6586: F, t7190: F, t2283: F, t9087: F, t1734: F, t1970: F, t209: F, t236: F, t476: F, t8577: F, t9159: F) -> (F, F, F, F, F) {
    let t45960 = t3351 * t3352 * t2144 * t6583;
    let t45964 = t3351 * t1971 * t7190 * t6586;
    let t45966 = t9087 * t2283;
    let t45974 = t1970 * t3352 * t236 * t1734 * t476 * t209;
    let t45976 = t8577 * t9159;
    (t45960, t45964, t45966, t45974, t45976)
}
