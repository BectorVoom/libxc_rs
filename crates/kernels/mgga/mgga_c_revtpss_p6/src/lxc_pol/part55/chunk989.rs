//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 989/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk989<F: Float>(t1256: F, t8185: F, t1238: F, t1791: F, t26827: F, t26855: F, t26863: F, t29047: F, t29055: F, t29062: F, t29065: F, t29069: F, t29072: F, t29074: F, t484: F, t5320: F, t7613: F) -> F {
    let t29077 = t8185 * t1256;
    let t29079 = t29047 * t29055 / F::new(216.0) - F::cast_from(0.42874018118069736972e-3_f64) * t26827 * t1791 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t5320 + F::cast_from(0.22866142996303859718e-2_f64) * t29062 * t1238 - F::cast_from(0.28582678745379824648e-3_f64) * t29065 - F::cast_from(0.19055119163586549765e-3_f64) * t26855 + F::cast_from(0.28582678745379824648e-3_f64) * t26863 - F::cast_from(0.22866142996303859718e-2_f64) * t29069 * t484 + F::cast_from(0.28582678745379824648e-3_f64) * t29072 + F::cast_from(0.42874018118069736972e-3_f64) * t29074 * t484 - F::cast_from(0.15244095330869239812e-2_f64) * t29077;
    t29079
}
