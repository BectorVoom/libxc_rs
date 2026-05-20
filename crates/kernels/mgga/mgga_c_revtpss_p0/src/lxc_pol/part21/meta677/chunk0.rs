//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2488/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2488<F: Float>(t43813: F, t43816: F, t3431: F, t408: F, t3434: F, t1126: F, t12247: F, t3800: F, t12773: F, t12784: F, t12772: F, t12835: F, t3625: F) -> (F, F, F, F, F, F, F, F) {
    let t44039 = F::cast_from(0.31003950617283950618e1_f64) * t43813;
    let t44040 = F::cast_from(0.13388493827160493828e1_f64) * t43816;
    let t44089 = t3431 * t3431;
    let t44091 = t408 / t44089;
    let t44092 = t3434 * t3434;
    let t44093 = F::new(1.0) / t44092;
    let t44101 = t1126 * t12247;
    let t44125 = t3800 * t3800;
    let t44126 = F::new(1.0) / t44125;
    let t44200 = t12784 * t12773;
    let t44215 = t3625 * t12772 * t12835;
    (t44039, t44040, t44091, t44093, t44101, t44126, t44200, t44215)
}
