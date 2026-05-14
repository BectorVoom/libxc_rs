//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 605/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk605<F: Float>(t318: F, t565: F, t86: F, t1520: F, t3393: F, t1523: F, t238: F, t3751: F, t41: F, t3754: F, t538: F, t1455: F, t531: F, t1444: F, t1466: F, t1527: F) -> (F, F, F, F, F, F, F, F) {
    let t4213 = 0.88437037037037037037e-2 * t86 * t318 * t565;
    let t4214 = t3393 * t1520;
    let t4217 = t86 * t238 * t1523;
    let t4219 = t41 * t3751;
    let t4220 = t538 * t3754;
    let t4225 = t1455 * t531;
    let t4230 = t538 * t1444;
    let t4248 = t1527 * t1466;
    (t4213, t4214, t4217, t4219, t4220, t4225, t4230, t4248)
}
