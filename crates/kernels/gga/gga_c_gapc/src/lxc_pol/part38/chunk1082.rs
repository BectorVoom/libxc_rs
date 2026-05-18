//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1082/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1082<F: Float>(t11969: F, t128: F, t7333: F, t875: F, t966: F, t11755: F, t655: F, t761: F, t11960: F, t28920: F, t871: F, t11961: F, t29108: F) -> (F, F, F, F) {
    let t33441 = t11969 * t7333 * t966 * t128 * t875;
    let t33444 = t761 * t655 * t11755;
    let t33447 = t871 * t11960 * t28920;
    let t33449 = t11961 * t29108;
    (t33441, t33444, t33447, t33449)
}
