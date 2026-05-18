//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 910/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk910<F: Float>(t1986: F, t5142: F, t675: F, t2289: F, t7944: F, t1971: F, t27326: F, t3351: F, t7262: F, t511: F, t618: F, t7231: F, t848: F) -> (F, F, F, F) {
    let t39715 = t675 * t1986 * t5142;
    let t39717 = t7944 * t2289;
    let t39721 = t3351 * t1971 * t7262 * t27326;
    let t39726 = t3351 * t7231 * t511 * t618 * t848;
    (t39715, t39717, t39721, t39726)
}
