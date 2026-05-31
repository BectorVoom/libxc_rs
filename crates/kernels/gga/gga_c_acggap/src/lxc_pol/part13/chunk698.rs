//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 698/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk698<F: Float>(t1061: F, t1095: F, t7476: F, t1980: F, t1988: F, t2109: F, t368: F, t7380: F, t355: F, t372: F, t1083: F, t2095: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7478 = t7476 * t1095 * t1061;
    let t7479 = t1980 * t7478;
    let t7481 = t1988 * t2109;
    let t7483 = t368 * t1061;
    let t7484 = t7380 * t7483;
    let t7485 = F::cast_from(0.4584375e-1_f64) * t7484;
    let t7486 = t355 * t372;
    let t7487 = t1083 * t7486;
    let t7488 = t2095 * t7487;
    (t7478, t7479, t7481, t7483, t7484, t7485, t7486, t7487, t7488)
}
