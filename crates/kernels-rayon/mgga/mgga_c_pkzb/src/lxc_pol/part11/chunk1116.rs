//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1116/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1116(t22233: f64, t2239: f64, t3030: f64, t1171: f64, t6198: f64, t2317: f64, t3113: f64, t1201: f64, t6230: f64, t2278: f64, t3080: f64, t1189: f64, t6287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22706 = 0.68493333333333333332e-1_f64 * t22233;
    let t22716 = 0.71233333333333333332e-1_f64 * t22233;
    let t22722 = t3030 * t2239;
    let t22727 = t1171 * t6198;
    let t22745 = t3113 * t2317;
    let t22750 = t1201 * t6230;
    let t22762 = t3080 * t2278;
    let t22767 = t1189 * t6287;
    (t22706, t22716, t22722, t22727, t22745, t22750, t22762, t22767)
}
