//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 483/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk483<F: Float>(t203: F, t2465: F, t2464: F, t587: F, t447: F, t487: F, t2365: F, t1416: F, t1421: F, t901: F, t1433: F, t586: F) -> (F, F, F, F, F, F, F, F) {
    let t2466 = t2465 * t203;
    let t2467 = t2464 * t2466;
    let t2468 = t587 * t2467;
    let t2470 = t487 * t447;
    let t2471 = t2365 * t2470;
    let t2472 = t1416 * t2471;
    let t2474 = t1421 * t901;
    let t2476 = t1433 * t586;
    (t2466, t2467, t2468, t2470, t2471, t2472, t2474, t2476)
}
