//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 753/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk753<F: Float>(t1457: F, t2645: F, t36516: F, t43464: F, t43467: F, t43470: F, t43522: F, t43526: F, t44707: F, t723: F) -> (F, F, F, F, F, F, F) {
    let t45356 = 0.42900587942220512003e1 * t36516 * t1457 * t2645;
    let t45357 = 0.11916829983950142223e0 * t43464;
    let t45358 = 0.11916829983950142223e0 * t43467;
    let t45359 = 0.11916829983950142223e0 * t43470;
    let t45366 = 0.59584149919750711116e-1 * t43522;
    let t45367 = 0.59584149919750711116e-1 * t43526;
    let t45369 = t44707 * t723;
    (t45356, t45357, t45358, t45359, t45366, t45367, t45369)
}
