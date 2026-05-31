//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 748/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk748<F: Float>(t1992: F, t407: F, t7842: F, t7585: F, t2070: F, t7839: F, t580: F, t7600: F, t56: F, t985: F, t569: F) -> (F, F, F, F, F, F, F) {
    let t7844 = t7842 * t1992 * t407;
    let t7845 = t7585 * t7844;
    let t7847 = t7839 * t2070;
    let t7848 = F::cast_from(0.21437009059034868486e-3_f64) * t7847;
    let t7849 = t7600 * t580;
    let t7850 = F::cast_from(77.0_f64) / F::cast_from(1728.0_f64) * t7849;
    let t7851 = t985 * t56;
    let t7852 = t7851 * t569;
    (t7844, t7845, t7847, t7848, t7850, t7851, t7852)
}
