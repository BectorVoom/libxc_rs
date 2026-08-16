//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1601/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1601<F: Float>(t3431: F, t408: F, t3434: F, t44018: F, t3427: F, t3433: F, t3435: F, t1126: F, t12247: F, t12249: F, t12227: F, t12243: F, t12364: F) -> (F, F, F, F, F, F) {
    let t44089 = t3431 * t3431;
    let t44091 = t408 / t44089;
    let t44092 = t3434 * t3434;
    let t44093 = F::cast_from(1.0_f64) / t44092;
    let t44096 = F::cast_from(0.24955700379505800916e5_f64) * t44091 * t44018 * t44093;
    let t44097 = t3427 * t3427;
    let t44100 = F::cast_from(0.48245938496077605201e2_f64) * t3433 * t44097 * t3435;
    let t44101 = t1126 * t12247;
    let t44103 = F::cast_from(0.3859675079686208416e3_f64) * t44101 * t12249;
    let t44106 = F::cast_from(0.57895126195293126241e3_f64) * t12227 * t44018 * t3435;
    let t44108 = F::cast_from(24.0_f64) * t12243 * t12364;
    (t44096, t44097, t44100, t44103, t44106, t44108)
}
