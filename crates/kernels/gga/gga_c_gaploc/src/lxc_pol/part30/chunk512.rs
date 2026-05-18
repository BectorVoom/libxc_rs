//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 512/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk512<F: Float>(t2617: F, t969: F, t2615: F, t590: F, t948: F, t1890: F, t935: F, t1423: F, t944: F) -> (F, F, F, F, F, F) {
    let t2618 = t969 * t2617;
    let t2619 = t2615 * t2618;
    let t2621 = t948 * t590;
    let t2624 = t1890 * t935;
    let t2625 = t2624 * t590;
    let t2628 = t1423 * t944;
    (t2618, t2619, t2621, t2624, t2625, t2628)
}
