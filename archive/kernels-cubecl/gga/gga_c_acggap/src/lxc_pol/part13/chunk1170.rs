//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1170/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1170<F: Float>(t36017: F, t525: F, t864: F, t1165: F, t31567: F, t604: F, t21099: F, t7337: F, t23688: F, t7346: F, t7310: F, t8771: F) -> (F, F, F, F, F, F) {
    let t36018 = F::cast_from(0.34299214494455789578e-2_f64) * t36017;
    let t36019 = t525 * t864;
    let t36022 = t31567 * t1165 * t604 * t36019;
    let t36026 = t7337 * t1165 * t604 * t21099;
    let t36030 = t7346 * t1165 * t604 * t23688;
    let t36031 = F::cast_from(0.31448092289604152068e-3_f64) * t36030;
    let t36032 = t7310 * t8771;
    (t36018, t36019, t36022, t36026, t36031, t36032)
}
