//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1683/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1683<F: Float>(t19049: F, t6223: F, t11465: F, t88008: F, t973: F, t981: F, t23696: F, t4719: F, t6227: F, t300: F, t88477: F, t23457: F) -> (F, F, F, F, F, F) {
    let t88580 = F::cast_from(0.35089341735807877242e1_f64) * t19049 * t6223;
    let t88584 = F::cast_from(0.14035736694323150897e2_f64) * t981 * t11465 * t88008 * t973;
    let t88586 = F::cast_from(0.23392894490538584828e1_f64) * t4719 * t23696;
    let t88588 = F::cast_from(0.10389515463408878255e3_f64) * t19049 * t6227;
    let t88590 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t88477;
    let t88592 = F::cast_from(0.14035736694323150897e2_f64) * t4719 * t23457;
    (t88580, t88584, t88586, t88588, t88590, t88592)
}
