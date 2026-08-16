//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1038/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1038(t2316: f64, t1275: f64, t2376: f64, t1004: f64, t6660: f64, t2182: f64, t775: f64, t253: f64, t5134: f64, t2568: f64, t3433: f64, t2563: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23193 = t2316 * t2316;
    let t23194 = 1.0_f64 / t23193;
    let t23495 = t2376 * t1275;
    let t23498 = t1004 * t6660;
    let t24039 = t2182 * t775;
    let t24063 = t5134 * t253;
    let t24521 = t3433 * t2568;
    let t24573 = t3433 * t2563;
    (t23194, t23495, t23498, t24039, t24063, t24521, t24573)
}
