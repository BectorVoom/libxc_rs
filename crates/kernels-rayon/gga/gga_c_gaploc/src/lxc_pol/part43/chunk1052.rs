//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1052/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1052(t14517: f64, t1960: f64, t42506: f64, t42509: f64, t44202: f64, t44207: f64, t44221: f64, t47096: f64, t47097: f64, t47105: f64, t47112: f64, t50930: f64, t50931: f64, t50933: f64, t50934: f64, t50983: f64, t50984: f64, t50985: f64, t50986: f64, t841: f64) -> f64 {
    let t51072 = 2.0_f64 * t14517 * t1960 * t841 - t42506 - t42509 + t44202 - t44207 - t44221 - 2.0_f64 * t47096 - 2.0_f64 * t47097 + 4.0_f64 * t47105 - 2.0_f64 * t47112 + t50930 + t50931 + t50933 - t50934 - t50983 - t50984 - t50985 - t50986;
    t51072
}
