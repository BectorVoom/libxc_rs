//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 762/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk762<F: Float>(t1330: F, t28: F, t271: F, t7553: F, t7557: F, t7491: F, t7766: F, t7490: F, t7552: F, t7558: F, t290: F, t35604: F) -> (F, F, F, F) {
    let t35613 = t28 * t1330;
    let t35616 = t7553 * t35613 * t271 * t7557;
    let t35618 = t7491 * t7766;
    let t35620 = t7490 * t7552;
    let t35621 = t35620 * t7558;
    let t35623 = t290 * t35604;
    (t35616, t35618, t35621, t35623)
}
