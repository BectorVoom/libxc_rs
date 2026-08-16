//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1425/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1425<F: Float>(t106951: F, t510: F, t652: F, t1774: F, t28017: F, t1845: F, t6347: F, t22574: F, t8643: F, t28831: F, t91655: F, t1983: F, t26167: F, t28834: F) -> (F, F, F, F, F) {
    let t107496 = F::cast_from(2.0_f64) * t652 * t510 * t106951;
    let t107499 = F::cast_from(6.0_f64) * t652 * t1774 * t28017;
    let t107504 = t6347 * t1845;
    let t107507 = F::cast_from(9.0_f64) * t22574 * t8643 * t107504;
    let t107509 = F::cast_from(18.0_f64) * t91655 * t28831;
    let t107512 = F::cast_from(9.0_f64) * t1983 * t26167 * t28834;
    (t107496, t107499, t107507, t107509, t107512)
}
