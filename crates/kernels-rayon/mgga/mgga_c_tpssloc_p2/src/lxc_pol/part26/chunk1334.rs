//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1334/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1334(t12739: f64, t6534: f64, t22479: f64, t5113: f64, t1401: f64, t81455: f64, t12521: f64, t3938: f64, t1873: f64, t3941: f64, t9416: f64, t16535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83966 = 6.0_f64 * t12739 * t6534;
    let t83968 = 6.0_f64 * t5113 * t22479;
    let t83979 = 0.135e2_f64 * t1401 * t81455;
    let t83984 = 0.405e2_f64 * t12521 * t6534;
    let t83988 = 0.405e2_f64 * t3938 * t22479;
    let t83991 = 27.0_f64 * t3941 * t1873 * t9416;
    let t83993 = 81.0_f64 * t16535 * t6534;
    (t83966, t83968, t83979, t83984, t83988, t83991, t83993)
}
