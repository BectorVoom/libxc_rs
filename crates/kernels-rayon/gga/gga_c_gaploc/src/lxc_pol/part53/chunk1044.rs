//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1044/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1044(t40744: f64, t43095: f64, t43099: f64, t43101: f64, t43106: f64, t43111: f64, t43115: f64, t43119: f64, t43122: f64, t43125: f64, t43131: f64, t43143: f64, t43146: f64, t43148: f64, t43152: f64, t43156: f64, t43157: f64, t47661: f64) -> f64 {
    let t51016 = -0.17090058289204942853e-2_f64 * t43095 + t43099 + t43101 - t47661 - t43106 + t43111 + t43115 - t43119 + t43122 - t43125 + t43131 - t43143 + t43146 + 0.12817543716903707139e-2_f64 * t40744 - t43148 - t43152 + t43156 + t43157;
    t51016
}
