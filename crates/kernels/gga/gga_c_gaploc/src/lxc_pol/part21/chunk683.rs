//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 683/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk683<F: Float>(t4082: F, t4085: F, t6372: F, t1250: F, t2280: F, t1254: F, t864: F, t6363: F, t6366: F, t6374: F, t2287: F, t471: F, t64: F, t869: F, t90: F) -> (F, F, F, F, F) {
    let t6377 = t4082 * t6372 * t4085;
    let t6379 = t2280 * t1250;
    let t6381 = t864 * t1254;
    let t6383 = F::new(189.0) / F::new(512.0) * t6363 - F::new(483.0) / F::new(16384.0) * t6366 + F::new(147.0) / F::new(1048576.0) * t6374 - F::new(49.0) / F::new(1048576.0) * t6377 + F::new(161.0) / F::new(16384.0) * t6379 - F::new(63.0) / F::new(512.0) * t6381;
    let t6393 = t6383 * t471 - F::new(8.0) / F::new(3.0) * t2287 * t64 + F::new(4.0) / F::new(3.0) * t869 * t90 + F::new(63.0) / F::new(512.0) * t6363 - F::new(49.0) / F::new(16384.0) * t6366 + F::new(49.0) / F::new(49152.0) * t6379 - F::new(21.0) / F::new(512.0) * t6381;
    (t6377, t6379, t6381, t6383, t6393)
}
