//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1089/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1089(t13740: f64, t484: f64, t42844: f64, t42845: f64, t42847: f64, t42850: f64, t42852: f64, t47024: f64, t47028: f64, t47032: f64, t47036: f64, t47040: f64) -> f64 {
    let t47042 = t484 * t13740;
    let t47044 = -t42844 + 0.56910013271352299198e-1_f64 * t47024 + 0.56910013271352299198e-1_f64 * t47028 - t42845 + t42847 + t42850 - 0.31616674039640166221e-2_f64 * t47032 - 0.63233348079280332442e-2_f64 * t42852 + 0.11856252764865062333e-2_f64 * t47036 + 0.28455006635676149599e-1_f64 * t47040 + 0.15808337019820083111e-2_f64 * t47042;
    t47044
}
