//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 647/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk647<F: Float>(t1061: F, t1095: F, t7476: F, t1980: F, t1988: F, t2109: F, t368: F, t7380: F, t355: F, t372: F, t1083: F, t2095: F, t2061: F, t361: F, t2060: F, t3360: F, t7336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7478 = t7476 * t1095 * t1061;
    let t7479 = t1980 * t7478;
    let t7481 = t1988 * t2109;
    let t7483 = t368 * t1061;
    let t7484 = t7380 * t7483;
    let t7485 = 0.4584375e-1 * t7484;
    let t7486 = t355 * t372;
    let t7487 = t1083 * t7486;
    let t7488 = t2095 * t7487;
    let t7489 = 0.305625e-1 * t7488;
    let t7490 = t361 * t2061;
    let t7491 = t2060 * t7490;
    let t7493 = t3360 * t7336;
    (t7478, t7479, t7481, t7483, t7484, t7485, t7486, t7487, t7488, t7489, t7490, t7491, t7493)
}
