//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1014/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1014<F: Float>(t1415: F, t48086: F, t2413: F, t42200: F, t42203: F, t42205: F, t42208: F, t42210: F, t42214: F, t42216: F, t42221: F, t42224: F, t42227: F, t48081: F, t13829: F, t1646: F, t528: F) -> (F, F) {
    let t48087 = t1415 * t48086;
    let t48088 = t48087 * t2413;
    let t48090 = -t42200 - t42203 - t42205 - t42208 - 0.19171462976960374838e0 * t48081 - 0.10725146985555128001e1 * t42210 - 0.10725146985555128001e1 * t42214 - 0.10725146985555128001e1 * t42216 - t42221 - t42224 - t42227 + 0.10725146985555128001e1 * t48088;
    let t48093 = 0.35750489951850426669e0 * t528 * t13829 * t1646;
    (t48090, t48093)
}
