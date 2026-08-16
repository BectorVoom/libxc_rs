//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1077/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1077(t4943: f64, t903: f64, t4940: f64, t2621: f64, t4939: f64, t3882: f64, t3886: f64, t4923: f64, t8752: f64, t11216: f64, t3770: f64, t10966: f64, t3811: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14842 = t4943 * t903;
    let t14845 = t4940 * t903;
    let t14848 = t4939 * t2621;
    let t14849 = t14848 * t903;
    let t14852 = t3886 * t3882;
    let t14855 = t4923 * t8752;
    let t14856 = t14855 * t903;
    let t14860 = 4.0_f64 * t11216 * t3770;
    let t14862 = 0.32163958997385070134e2_f64 * t10966 * t3811;
    (t14842, t14845, t14849, t14852, t14856, t14860, t14862)
}
