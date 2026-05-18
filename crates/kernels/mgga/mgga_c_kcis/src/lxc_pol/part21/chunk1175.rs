//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1175/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1175<F: Float>(t10496: F, t364: F, t392: F, t11229: F, t1278: F, t3662: F, t3668: F, t3621: F, t11181: F, t413: F, t429: F, t11182: F, t1236: F) -> (F, F, F, F, F, F) {
    let t33862 = t364 / t10496 / t392;
    let t34650 = t1278 * t11229;
    let t34662 = t3662 * t3668;
    let t34689 = t3621 * t3621;
    let t34690 = F::new(1.0) / t34689;
    let t34814 = t413 / t11181 / t429;
    let t35547 = t1236 * t11182;
    (t33862, t34650, t34662, t34690, t34814, t35547)
}
