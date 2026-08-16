//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1357/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1357<F: Float>(t22252: F, t6028: F, t6027: F, t20925: F, t4293: F, t4292: F, t17505: F, t5916: F, t17450: F, t2039: F, t5913: F, t21804: F, t4261: F) -> (F, F, F, F, F, F) {
    let t22361 = t6028 * t22252;
    let t22362 = t6027 * t22361;
    let t22364 = t4293 * t20925;
    let t22365 = t4292 * t22364;
    let t22367 = t17505 * t5916;
    let t22369 = t17450 * t2039;
    let t22371 = t17505 * t5913;
    let t22373 = t4261 * t21804;
    (t22362, t22365, t22367, t22369, t22371, t22373)
}
