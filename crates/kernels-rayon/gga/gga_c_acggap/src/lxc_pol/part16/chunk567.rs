//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 567/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk567(t1140: f64, t1511: f64, t1137: f64, t1494: f64, t1498: f64, t1032: f64, t1352: f64, t1446: f64, t997: f64, t1008: f64, t1418: f64, t1347: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4649 = 7.0_f64 / 144.0_f64 * t1140 * t1511;
    let t4651 = 7.0_f64 / 72.0_f64 * t1137 * t1494;
    let t4653 = 7.0_f64 / 72.0_f64 * t1137 * t1498;
    let t4670 = t1032 * t1352;
    let t4673 = 0.16006300097412701803e-1_f64 * t997 * t1446;
    let t4675 = 0.34299214494455789578e-2_f64 * t1008 * t1418;
    let t4677 = 0.34299214494455789578e-2_f64 * t1008 * t1347;
    (t4649, t4651, t4653, t4670, t4673, t4675, t4677)
}
