//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 566/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk566<F: Float>(t1140: F, t1507: F, t145: F, t1487: F, t3570: F, t500: F, t3573: F, t515: F, t435: F, t506: F) -> (F, F, F, F, F) {
    let t4629 = F::new(7.0) / F::new(144.0) * t1140 * t1507;
    let t4630 = t1487 * t145;
    let t4635 = t3570 * t500;
    let t4637 = t3573 * t515;
    let t4643 = t506 * t435;
    (t4629, t4630, t4635, t4637, t4643)
}
