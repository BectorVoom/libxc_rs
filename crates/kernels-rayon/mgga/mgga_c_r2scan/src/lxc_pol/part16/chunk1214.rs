//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1214/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1214(t11693: f64, t8198: f64, t10856: f64, t9319: f64, t38055: f64, t40042: f64, t40044: f64, t40048: f64, t40051: f64, t40054: f64, t40077: f64, t40087: f64, t40090: f64, t41680: f64) -> f64 {
    let t43488 = t8198 * t11693;
    let t43490 = t10856 * t9319;
    let t43493 = t40042 + 0.13972381860938637374e0_f64 * t40044 + t40048 + t40051 - t40054 + 0.17336443480108537126e0_f64 * t43488 - t41680 + t40077 - t38055 + 0.97574405393827830187e-2_f64 * t43490 - t40087 + 0.55889527443754549496e0_f64 * t40090;
    t43493
}
