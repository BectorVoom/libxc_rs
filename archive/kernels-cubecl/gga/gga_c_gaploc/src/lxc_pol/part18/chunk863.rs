//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 863/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk863<F: Float>(t550: F, t8528: F, t549: F, t1402: F, t2954: F, t2963: F, t590: F, t701: F, t1457: F, t8512: F, t3039: F, t783: F) -> (F, F, F, F, F, F, F, F) {
    let t8529 = t550 * t8528;
    let t8530 = t549 * t8529;
    let t8535 = t1402 * t2954;
    let t8540 = t2963 * t590;
    let t8549 = t8528 * t701;
    let t8550 = t1457 * t8549;
    let t8553 = t1457 * t8512;
    let t8556 = t3039 * t783;
    (t8529, t8530, t8535, t8540, t8549, t8550, t8553, t8556)
}
