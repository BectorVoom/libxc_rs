//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1029/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1029<F: Float>(t35975: F, t1554: F, t30644: F, t1558: F, t4326: F, t7647: F, t1421: F, t1983: F, t30827: F, t7586: F, t1545: F, t31824: F, t1416: F, t1992: F, t30154: F, t1345: F, t30148: F, t7842: F) -> (F, F, F, F, F, F, F, F) {
    let t35976 = 0.17149607247227894789e-2 * t35975;
    let t35977 = t30644 * t1554;
    let t35978 = 0.17149607247227894789e-2 * t35977;
    let t35979 = t30644 * t1558;
    let t35980 = 0.85748036236139473944e-3 * t35979;
    let t35981 = t7647 * t4326;
    let t35982 = 0.85748036236139473944e-3 * t35981;
    let t35985 = t30827 * t7586 * t1983 * t1421;
    let t35987 = t31824 * t1545;
    let t35988 = 0.34299214494455789578e-2 * t35987;
    let t35991 = t30154 * t7586 * t1992 * t1416;
    let t35992 = 0.20965394859736101378e-2 * t35991;
    let t35995 = t30154 * t7842 * t30148 * t1345;
    (t35976, t35978, t35980, t35982, t35985, t35988, t35992, t35995)
}
