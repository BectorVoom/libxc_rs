//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1127/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1127<F: Float>(t28876: F, t449: F, t446: F, t448: F, t6260: F, t2233: F, t1640: F, t1884: F, t5406: F, t637: F, t1881: F, t7892: F) -> (F, F, F, F, F) {
    let t28877 = t449 * t28876;
    let t28878 = t446 * t28877;
    let t28880 = t448 * t6260;
    let t28881 = t2233 * t28880;
    let t28883 = t1884 * t1640;
    let t28884 = t2233 * t28883;
    let t28886 = t5406 * t637;
    let t28887 = t2233 * t28886;
    let t28889 = t1881 * t7892;
    (t28878, t28881, t28884, t28887, t28889)
}
