//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 455/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk455<F: Float>(t43: F, t50: F, t560: F, t1690: F, t1694: F, t292: F, t817: F, t1699: F, t1702: F, t296: F, t829: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1717 = t560 * t560;
    let t1726 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t817 * t1690 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t292 * t1694);
    let t1732 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t829 * t1699 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t296 * t1702);
    let t1734 = t1726 / F::cast_from(2.0_f64) + t1732 / F::cast_from(2.0_f64);
    (t1717, t1734)
}
