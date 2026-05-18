//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1198/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1198<F: Float>(t47877: F, t587: F, t912: F, t1: F, t47008: F, t1415: F, t2413: F, t42200: F, t42203: F, t42205: F, t42208: F, t42210: F, t42214: F, t42216: F, t42221: F, t42224: F, t42227: F) -> (F, F) {
    let t48081 = t587 * t912 * t47877;
    let t48086 = t47008 * t1;
    let t48087 = t1415 * t48086;
    let t48088 = t48087 * t2413;
    let t48090 = -t42200 - t42203 - t42205 - t42208 - F::new(0.19171462976960374838e0) * t48081 - F::new(0.10725146985555128001e1) * t42210 - F::new(0.10725146985555128001e1) * t42214 - F::new(0.10725146985555128001e1) * t42216 - t42221 - t42224 - t42227 + F::new(0.10725146985555128001e1) * t48088;
    (t48086, t48090)
}
