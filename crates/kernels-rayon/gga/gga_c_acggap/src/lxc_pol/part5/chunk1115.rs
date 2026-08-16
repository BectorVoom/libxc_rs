//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1115/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1115(t11731: f64, t11733: f64, t14941: f64, t11748: f64, t11750: f64, t11762: f64, t229: f64, t6012: f64, t224: f64, t6008: f64, t14957: f64, t14959: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19977 = 0.5848223622634646207e0_f64 * t11731;
    let t19978 = 0.17315859105681463759e2_f64 * t11733;
    let t19979 = 80.0_f64 * t14941;
    let t19980 = 8.0_f64 * t11748;
    let t19981 = 20.0_f64 * t11750;
    let t19982 = 32.0_f64 * t11762;
    let t19983 = t229 * t6012;
    let t19984 = 8.0_f64 * t19983;
    let t19985 = t224 * t6012;
    let t19986 = 8.0_f64 * t19985;
    let t19987 = t229 * t6008;
    let t19988 = 8.0_f64 * t19987;
    let t19989 = 24.0_f64 * t14957;
    let t19990 = 24.0_f64 * t14959;
    (t19977, t19978, t19979, t19980, t19981, t19982, t19984, t19986, t19988, t19989, t19990)
}
