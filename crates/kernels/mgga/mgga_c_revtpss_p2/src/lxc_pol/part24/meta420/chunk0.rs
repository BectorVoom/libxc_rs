//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1368/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1368<F: Float>(t43813: F, t43816: F, t3431: F, t408: F, t3434: F, t3800: F, t3362: F, t3603: F, t13100: F, t828: F, t12879: F, t12256: F, t3698: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44039 = F::cast_from(0.31003950617283950618e1_f64) * t43813;
    let t44040 = F::cast_from(0.13388493827160493828e1_f64) * t43816;
    let t44089 = t3431 * t3431;
    let t44091 = t408 / t44089;
    let t44092 = t3434 * t3434;
    let t44093 = F::cast_from(1.0_f64) / t44092;
    let t44125 = t3800 * t3800;
    let t44126 = F::cast_from(1.0_f64) / t44125;
    let t44190 = t3603 * t3362;
    let t44225 = t828 * t13100;
    let t44250 = t828 * t12879;
    let t44307 = F::cast_from(0.86419753086419753087e-1_f64) * t43813;
    let t44348 = t3698 * t12256;
    (t44039, t44040, t44091, t44093, t44126, t44190, t44225, t44250, t44307, t44348)
}
