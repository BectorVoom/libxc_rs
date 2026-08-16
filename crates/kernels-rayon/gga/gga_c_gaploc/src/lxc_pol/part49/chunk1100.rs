//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1100/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1100(t13949: f64, t747: f64, t1960: f64, t2728: f64, t3749: f64, t2358: f64, t39337: f64, t42506: f64, t42509: f64, t44196: f64, t44198: f64, t44202: f64, t44203: f64, t47075: f64, t47078: f64, t841: f64) -> (f64, f64) {
    let t47102 = t13949 * t747;
    let t47105 = t1960 * t3749 * t2728;
    let t47107 = t39337 * t2358;
    let t47108 = 2.0_f64 * t47107;
    let t47109 = -t47102 * t841 - t42506 - t42509 + 2.0_f64 * t44196 - t44198 + t44202 - t44203 + t47075 - t47078 + 2.0_f64 * t47105 - t47108;
    (t47108, t47109)
}
