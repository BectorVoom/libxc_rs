//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 852/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk852(t12064: f64, t540: f64, t1: f64, t106: f64, t12000: f64, t192: f64, t12078: f64, t1397: f64, t12323: f64, t747: f64, t1959: f64, t3730: f64) -> (f64, f64, f64, f64, f64) {
    let t38688 = t12064 * t540;
    let t38759 = t12000 * t1 * t106 * t192;
    let t38770 = t1397 * t12078;
    let t38885 = t12323 * t747;
    let t38892 = t3730 * t1959;
    (t38688, t38759, t38770, t38885, t38892)
}
