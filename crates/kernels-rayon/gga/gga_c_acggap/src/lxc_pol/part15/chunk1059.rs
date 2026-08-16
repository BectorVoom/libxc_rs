//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1059/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1059(t35963: f64, t35967: f64, t35969: f64, t35973: f64, t35975: f64, t35977: f64, t35979: f64, t35981: f64, t35987: f64, t35997: f64, t36006: f64, t36010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37808 = 0.34299214494455789578e-2_f64 * t35963;
    let t37810 = 0.13719685797782315831e-1_f64 * t35967;
    let t37811 = 0.16006300097412701803e-1_f64 * t35969;
    let t37813 = 0.16006300097412701803e-1_f64 * t35973;
    let t37814 = 0.34299214494455789578e-2_f64 * t35975;
    let t37815 = 0.34299214494455789578e-2_f64 * t35977;
    let t37816 = 0.17149607247227894789e-2_f64 * t35979;
    let t37817 = 0.17149607247227894789e-2_f64 * t35981;
    let t37819 = 0.68598428988911579156e-2_f64 * t35987;
    let t37822 = 0.18868855373762491241e-1_f64 * t35997;
    let t37826 = 0.34299214494455789578e-2_f64 * t36006;
    let t37827 = 0.20965394859736101379e-2_f64 * t36010;
    (t37808, t37810, t37811, t37813, t37814, t37815, t37816, t37817, t37819, t37822, t37826, t37827)
}
