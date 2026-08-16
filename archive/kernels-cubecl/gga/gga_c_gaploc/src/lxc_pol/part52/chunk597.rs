//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 597/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk597<F: Float>(t11318: F, t475: F, t1445: F, t11172: F, t11260: F, t1457: F, t11255: F, t11264: F, t188: F, t11271: F, t1589: F, t3541: F) -> (F, F, F, F, F, F, F, F) {
    let t11342 = t11318 * t475;
    let t11343 = t1445 * t11342;
    let t11346 = t11172 * t475;
    let t11347 = t1445 * t11346;
    let t11350 = t1445 * t11260;
    let t11353 = t1457 * t11260;
    let t11356 = t1457 * t11255;
    let t11359 = t188 * t11264;
    let t11362 = t188 * t11271;
    let t11365 = t1589 * t3541;
    (t11343, t11347, t11350, t11353, t11356, t11359, t11362, t11365)
}
