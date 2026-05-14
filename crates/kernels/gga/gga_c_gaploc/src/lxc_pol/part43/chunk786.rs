//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 786/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk786<F: Float>(t43454: F, t13118: F, t15362: F, t2365: F, t32357: F, t6111: F, t32436: F, t24501: F, t825: F, t9438: F, t13038: F, t2194: F, t10627: F, t2530: F) -> (F, F, F, F, F, F, F) {
    let t43455 = 0.17875244975925213335e0 * t43454;
    let t43464 = t15362 * t13118;
    let t43465 = 0.59584149919750711116e-1 * t43464;
    let t43467 = t6111 * t2365 * t32357;
    let t43468 = 0.59584149919750711116e-1 * t43467;
    let t43470 = t6111 * t2365 * t32436;
    let t43471 = 0.59584149919750711116e-1 * t43470;
    let t43476 = t825 * t9438 * t24501;
    let t43477 = 0.31952438294933958064e-1 * t43476;
    let t43479 = 0.92023022289409799224e1 * t2194 * t13038;
    let t43486 = t10627 * t2530;
    (t43455, t43465, t43468, t43471, t43477, t43479, t43486)
}
