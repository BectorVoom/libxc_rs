//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1360/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1360<F: Float>(t31590: F, t493: F, t1441: F, t590: F, t2897: F, t4398: F, t7030: F, t1415: F, t8330: F, t2365: F, t25740: F, t7025: F) -> (F, F, F, F) {
    let t34273 = t493 * t31590;
    let t34276 = F::new(0.2044956050875773316e1) * t1441 * t34273 * t590;
    let t34278 = t4398 * t2897 * t7030;
    let t34279 = F::new(0.29792074959875355558e-1) * t34278;
    let t34281 = t1415 * t8330 * t7030;
    let t34282 = F::new(0.29792074959875355558e-1) * t34281;
    let t34284 = t7025 * t2365 * t25740;
    (t34276, t34279, t34282, t34284)
}
