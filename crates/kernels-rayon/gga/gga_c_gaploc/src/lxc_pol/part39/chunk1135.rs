//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1135/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1135(t13879: f64, t2009: f64, t773: f64, t38950: f64, t955: f64, t43904: f64, t43908: f64, t43909: f64, t43910: f64, t43911: f64, t43913: f64, t43915: f64, t43918: f64, t43919: f64, t43922: f64) -> f64 {
    let t47430 = 0.35750489951850426669e0_f64 * t773 * t13879 * t2009;
    let t47432 = t955 * t38950;
    let t47436 = -t47430 - 0.25561950635947166451e0_f64 * t43904 + t43908 + 0.23833659967900284446e0_f64 * t47432 - t43909 + t43910 + t43911 - t43913 + t43915 + t43918 - 0.19171462976960374838e0_f64 * t43919 - 0.19171462976960374838e0_f64 * t43922;
    t47436
}
