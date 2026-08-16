//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 998/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk998(t12019: f64, t374: f64, t11657: f64, t11660: f64, t11687: f64, t11700: f64, t11753: f64, t11758: f64, t11762: f64, t11766: f64, t11772: f64, t11774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12020 = t12019 * t374;
    let t12120 = 0.23115257973478049502e0_f64 * t11657;
    let t12121 = 0.46574606203128791246e-1_f64 * t11660;
    let t12132 = 0.23115257973478049502e0_f64 * t11687;
    let t12138 = 0.14282990759302185292e-1_f64 * t11700;
    let t12158 = 0.19514881078765566037e-1_f64 * t11753;
    let t12162 = 0.54878743191129263322e-2_f64 * t11758;
    let t12163 = 0.46574606203128791246e-1_f64 * t11762;
    let t12164 = 0.13972381860938637374e0_f64 * t11766;
    let t12166 = 0.46574606203128791246e-1_f64 * t11772;
    let t12167 = 0.10975748638225852664e-1_f64 * t11774;
    (t12020, t12120, t12121, t12132, t12138, t12158, t12162, t12163, t12164, t12166, t12167)
}
