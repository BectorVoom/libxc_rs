//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1198/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1198(t47877: f64, t587: f64, t912: f64, t1: f64, t47008: f64, t1415: f64, t2413: f64, t42200: f64, t42203: f64, t42205: f64, t42208: f64, t42210: f64, t42214: f64, t42216: f64, t42221: f64, t42224: f64, t42227: f64) -> (f64, f64) {
    let t48081 = t587 * t912 * t47877;
    let t48086 = t47008 * t1;
    let t48087 = t1415 * t48086;
    let t48088 = t48087 * t2413;
    let t48090 = -t42200 - t42203 - t42205 - t42208 - 0.19171462976960374838e0_f64 * t48081 - 0.10725146985555128001e1_f64 * t42210 - 0.10725146985555128001e1_f64 * t42214 - 0.10725146985555128001e1_f64 * t42216 - t42221 - t42224 - t42227 + 0.10725146985555128001e1_f64 * t48088;
    (t48086, t48090)
}
