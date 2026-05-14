//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 303/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk303<F: Float>(t123: F, t2536: F, t734: F, t795: F, t935: F, t740: F, t731: F, t945: F, t679: F, t78: F, t278: F, t481: F) -> (F, F, F, F, F, F, F) {
    let t2537 = t2536 * t123;
    let t2538 = t2537 * t734;
    let t2541 = t795 * t935;
    let t2542 = t2541 * t740;
    let t2545 = t731 * t945;
    let t2547 = t78 * t679;
    let t2549 = t481 * t2547 * t278;
    (t2537, t2538, t2541, t2542, t2545, t2547, t2549)
}
