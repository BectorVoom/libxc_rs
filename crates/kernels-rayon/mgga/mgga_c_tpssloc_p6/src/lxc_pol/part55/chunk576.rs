//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 576/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk576(t4493: f64, t959: f64, t1580: f64, t2929: f64, t2932: f64, t950: f64, t1592: f64, t2970: f64, t973: f64, t2978: f64, t60: f64, t344: f64) -> (f64, f64, f64, f64) {
    let t4495 = 0.5848223622634646207e0_f64 * t959 * t4493;
    let t4496 = t2929 * t1580;
    let t4497 = t2932 * t950;
    let t4498 = t4496 * t4497;
    let t4500 = 0.17315859105681463759e2_f64 * t959 * t4498;
    let t4506 = t2970 * t1592;
    let t4507 = t973 * t4506;
    let t4509 = t60 * t2978;
    let t4510 = t4509 * t344;
    (t4495, t4500, t4507, t4510)
}
