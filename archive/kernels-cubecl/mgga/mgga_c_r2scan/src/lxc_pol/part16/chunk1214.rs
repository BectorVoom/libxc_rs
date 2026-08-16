//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1214/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1214<F: Float>(t11693: F, t8198: F, t10856: F, t9319: F, t38055: F, t40042: F, t40044: F, t40048: F, t40051: F, t40054: F, t40077: F, t40087: F, t40090: F, t41680: F) -> F {
    let t43488 = t8198 * t11693;
    let t43490 = t10856 * t9319;
    let t43493 = t40042 + F::cast_from(0.13972381860938637374e0_f64) * t40044 + t40048 + t40051 - t40054 + F::cast_from(0.17336443480108537126e0_f64) * t43488 - t41680 + t40077 - t38055 + F::cast_from(0.97574405393827830187e-2_f64) * t43490 - t40087 + F::cast_from(0.55889527443754549496e0_f64) * t40090;
    t43493
}
