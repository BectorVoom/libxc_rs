//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1232/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1232<F: Float>(t7898: F, t94491: F, t3245: F, t7932: F, t7935: F, t10470: F, t2244: F, t2237: F, t2238: F, t737: F, t61287: F, t7907: F) -> (F, F, F, F, F, F, F) {
    let t94524 = t7898 * t94491;
    let t94537 = t3245 * t7932;
    let t94539 = t3245 * t7935;
    let t94588 = t10470 * t2244;
    let t94589 = F::new(0.73697530864197530862e-3) * t94588;
    let t94614 = F::new(0.25742669753086419753e-3) * t2237 * t737 * t2238;
    let t94626 = t7907 * t61287;
    (t94524, t94537, t94539, t94588, t94589, t94614, t94626)
}
