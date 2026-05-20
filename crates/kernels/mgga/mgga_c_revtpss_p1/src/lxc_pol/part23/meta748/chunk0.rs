//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2537/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2537<F: Float>(t11466: F, t300: F, t51973: F, t52035: F, t52037: F, t1633: F, t3012: F, t2986: F, t4682: F, t11465: F, t1626: F, t11509: F, t4707: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t52238 = t300 * t11466;
    let t52337 = F::cast_from(0.12361111111111111111e-1_f64) * t51973;
    let t52346 = F::cast_from(0.24722222222222222222e-1_f64) * t52035;
    let t52397 = F::cast_from(0.2283111111111111111e-1_f64) * t51973;
    let t52406 = F::cast_from(0.4566222222222222222e-1_f64) * t52035;
    let t52407 = F::cast_from(0.1522074074074074074e-1_f64) * t52037;
    let t52430 = t3012 * t1633;
    let t52440 = t4682 * t2986;
    let t52443 = t1626 * t11465;
    let t52459 = t4707 * t11509;
    (t52238, t52337, t52346, t52397, t52406, t52407, t52430, t52440, t52443, t52459)
}
