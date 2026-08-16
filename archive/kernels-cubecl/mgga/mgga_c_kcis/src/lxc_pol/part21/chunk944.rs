//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 944/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk944<F: Float>(t13467: F, t14347: F, t13516: F, t4565: F, t1662: F, t2952: F, t3269: F, t4621: F, t934: F, t3096: F, t3274: F, t1045: F) -> (F, F, F, F, F, F) {
    let t14348 = t14347 * t13467;
    let t14351 = t4565 * t13516;
    let t14355 = t3269 * t1662 * t2952;
    let t14359 = t3269 * t4621 * t934;
    let t14363 = t3274 * t1662 * t3096;
    let t14367 = t3274 * t4621 * t1045;
    (t14348, t14351, t14355, t14359, t14363, t14367)
}
