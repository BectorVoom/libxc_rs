//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 628/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk628<F: Float>(t7201: F, t655: F, t1769: F, t2449: F, t1772: F, t2448: F, sigma2: F) -> (F, F, F, F) {
    let t7202 = t7201 * sigma2;
    let t7203 = t7202 * t655;
    let t7206 = t2449 * t1769;
    let t7208 = t2448 * t1772;
    (t7202, t7203, t7206, t7208)
}
