//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 689/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk689<F: Float>(t209: F, t24: F, t1299: F, t637: F, t1640: F, t448: F, t1300: F, t2272: F, t1598: F, t3964: F) -> (F, F, F, F, F) {
    let t7783 = t209 * t24;
    let t7886 = t1299 * t637;
    let t7889 = t448 * t1640;
    let t7892 = t1300 * t2272;
    let t7895 = t3964 * t1598;
    (t7783, t7886, t7889, t7892, t7895)
}
