//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1150/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1150<F: Float>(t1181: F, t4762: F, t7351: F, t7564: F, t1562: F, t31824: F, t1449: F, t30148: F, t30159: F, t7586: F, t1541: F, t31611: F) -> (F, F, F, F) {
    let t35782 = t7564 * t1181 * t7351 * t4762;
    let t35784 = t31824 * t1562;
    let t35785 = F::cast_from(0.34299214494455789578e-2_f64) * t35784;
    let t35788 = t30159 * t7586 * t30148 * t1449;
    let t35789 = F::cast_from(0.12579236915841660827e-2_f64) * t35788;
    let t35790 = t31611 * t1541;
    (t35782, t35785, t35789, t35790)
}
