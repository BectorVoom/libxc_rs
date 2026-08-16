//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 684/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk684<F: Float>(t4082: F, t4085: F, t6372: F, t1250: F, t2280: F, t1254: F, t864: F, t6363: F, t6366: F, t6374: F, t2287: F, t471: F, t64: F, t869: F, t90: F) -> (F, F, F, F) {
    let t6377 = t4082 * t6372 * t4085;
    let t6379 = t2280 * t1250;
    let t6381 = t864 * t1254;
    let t6383 = F::cast_from(189.0_f64) / F::cast_from(512.0_f64) * t6363 - F::cast_from(483.0_f64) / F::cast_from(16384.0_f64) * t6366 + F::cast_from(147.0_f64) / F::cast_from(1048576.0_f64) * t6374 - F::cast_from(49.0_f64) / F::cast_from(1048576.0_f64) * t6377 + F::cast_from(161.0_f64) / F::cast_from(16384.0_f64) * t6379 - F::cast_from(63.0_f64) / F::cast_from(512.0_f64) * t6381;
    let t6393 = t6383 * t471 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2287 * t64 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t869 * t90 + F::cast_from(63.0_f64) / F::cast_from(512.0_f64) * t6363 - F::cast_from(49.0_f64) / F::cast_from(16384.0_f64) * t6366 + F::cast_from(49.0_f64) / F::cast_from(49152.0_f64) * t6379 - F::cast_from(21.0_f64) / F::cast_from(512.0_f64) * t6381;
    (t6377, t6379, t6381, t6393)
}
