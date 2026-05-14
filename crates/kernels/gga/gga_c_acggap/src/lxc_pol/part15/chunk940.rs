//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 940/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk940<F: Float>(t35930: F, t35934: F, t35951: F, t35961: F, t35963: F, t35967: F, t35969: F, t35973: F, t35975: F, t35977: F, t35979: F, t35981: F, t35987: F, t35997: F, t36006: F, t36010: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37789 = 0.85748036236139473944e-3 * t35930;
    let t37790 = 0.42874018118069736972e-3 * t35934;
    let t37801 = 0.34299214494455789578e-2 * t35951;
    let t37807 = 0.34299214494455789578e-2 * t35961;
    let t37808 = 0.34299214494455789578e-2 * t35963;
    let t37810 = 0.13719685797782315831e-1 * t35967;
    let t37811 = 0.16006300097412701803e-1 * t35969;
    let t37813 = 0.16006300097412701803e-1 * t35973;
    let t37814 = 0.34299214494455789578e-2 * t35975;
    let t37815 = 0.34299214494455789578e-2 * t35977;
    let t37816 = 0.17149607247227894789e-2 * t35979;
    let t37817 = 0.17149607247227894789e-2 * t35981;
    let t37819 = 0.68598428988911579156e-2 * t35987;
    let t37822 = 0.18868855373762491241e-1 * t35997;
    let t37826 = 0.34299214494455789578e-2 * t36006;
    let t37827 = 0.20965394859736101379e-2 * t36010;
    (t37789, t37790, t37801, t37807, t37808, t37810, t37811, t37813, t37814, t37815, t37816, t37817, t37819, t37822, t37826, t37827)
}
