//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 345/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk345<F: Float>(t43: F, t50: F, t474: F, t886: F, t34: F, t47: F, t234: F, t821: F, t478: F, t893: F, t52: F, t238: F, t59: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1361 = t886 * t474;
    let t1364 = t47 * t34;
    let t1368 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1361 * t234 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1364 * t821);
    let t1369 = t893 * t478;
    let t1372 = t52 * t34;
    let t1376 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1369 * t238 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1372 * t821);
    let t1378 = (t1368 + t1376) * t59;
    (t1361, t1364, t1369, t1372, t1378)
}
