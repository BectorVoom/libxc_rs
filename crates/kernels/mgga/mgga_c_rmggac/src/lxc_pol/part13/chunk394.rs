//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 394/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk394<F: Float>(t2463: F, t305: F, t558: F, t699: F, t326: F, t118: F, t2435: F, t2447: F, t338: F) -> (F, F, F, F, F) {
    let t2464 = t305 * t2463;
    let t2466 = t699 * t558;
    let t2467 = t326 * t2466;
    let t2469 = t118 * t2435;
    let t2471 = t338 * t2447;
    (t2464, t2466, t2467, t2469, t2471)
}
