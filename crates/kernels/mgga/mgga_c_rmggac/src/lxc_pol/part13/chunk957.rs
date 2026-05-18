//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 957/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk957<F: Float>(t39684: F, t5162: F, t2067: F, t26: F, t25525: F, t5163: F, t649: F, t35960: F, t5166: F, t2079: F, t262: F, t570: F, t830: F) -> (F, F, F, F, F, F) {
    let t40993 = t5162 * t39684;
    let t40998 = t2067 * t26;
    let t40999 = t25525 * t40998;
    let t41000 = t649 * t5163;
    let t41001 = t40999 * t41000;
    let t41004 = t35960 * t649 * t5166;
    let t41021 = t2079 * t262 * t830 * t570;
    (t40993, t40998, t41000, t41001, t41004, t41021)
}
