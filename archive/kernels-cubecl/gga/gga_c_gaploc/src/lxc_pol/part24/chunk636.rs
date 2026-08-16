//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 636/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk636<F: Float>(t1535: F, t4416: F, t584: F, t585: F, t1406: F, t1435: F, t121: F, t1508: F, t1397: F, t1420: F, t1: F, t4149: F) -> (F, F, F, F, F, F) {
    let t4417 = t1535 * t4416;
    let t4418 = t584 * t4417;
    let t4421 = t585 * t4416;
    let t4425 = t584 * t4421;
    let t4428 = t1406 * t1435;
    let t4461 = t1508 * t121;
    let t4494 = t1397 * t1420;
    let t4501 = t4149 * t1;
    (t4418, t4425, t4428, t4461, t4494, t4501)
}
