//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 946/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk946<F: Float>(t29789: F, t488: F, t3238: F, t6557: F, t1332: F, t16246: F, t22943: F, t4551: F, t23249: F, t4572: F, t11490: F, t4462: F, t5717: F, t1909: F, t4454: F, t3193: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29790 = t488 * t29789;
    let t29792 = t3238 * t6557;
    let t29794 = t16246 * t1332;
    let t29796 = t22943 * t4551;
    let t29798 = t23249 * t4572;
    let t29799 = t11490 * t29798;
    let t29802 = t5717 * t4462;
    let t29803 = t1909 * t29802;
    let t29806 = t5717 * t4454;
    let t29807 = t3193 * t29806;
    (t29790, t29792, t29794, t29796, t29798, t29799, t29802, t29803, t29806, t29807)
}
