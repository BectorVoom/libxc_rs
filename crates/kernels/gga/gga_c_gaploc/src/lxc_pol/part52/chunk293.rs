//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 293/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk293<F: Float>(t2366: F, t475: F, t2365: F, t1429: F, t1: F, t2299: F, t544: F, t2339: F, t549: F, t2334: F, t1445: F, t2345: F) -> (F, F, F, F, F, F) {
    let t2367 = t2366 * t475;
    let t2368 = t2365 * t2367;
    let t2369 = t1429 * t2368;
    let t2371 = t2299 * t1;
    let t2372 = t544 * t2371;
    let t2375 = t549 * t2339;
    let t2378 = t2334 * t475;
    let t2379 = t1445 * t2378;
    let t2382 = t1445 * t2345;
    (t2369, t2371, t2372, t2375, t2379, t2382)
}
