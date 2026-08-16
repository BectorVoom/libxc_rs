//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1360/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1360(t1851: f64, t30045: f64, t3620: f64, t5329: f64, t15171: f64, t5310: f64, t922: f64, t7794: f64, t993: f64, t2888: f64, t27028: f64, t15256: f64, t15481: f64, t15501: f64, t26955: f64, t26960: f64, t7772: f64, t92851: f64, t95928: f64, t95931: f64, t95946: f64, t96799: f64, t96995: f64, t96999: f64) -> (f64, f64, f64) {
    let t97069 = t5329 * t30045 * t1851 * t3620;
    let t97076 = t5310 * t15171 * t922;
    let t97083 = t993 * t7794;
    let t97089 = t2888 * t7794;
    let t97093 = t993 * t27028;
    let t97098 = 0.13913205078125e-3_f64 * t7772 * t97069 + 0.10306077835648148148e-4_f64 * t92851 - 0.30952962962962962962e-2_f64 * t95928 + 0.25794135802469135802e-2_f64 * t95931 + 0.30918233506944444444e-4_f64 * t26955 * t97076 + 0.15459116753472222222e-4_f64 * t26955 * t96995 + 0.20612155671296296296e-4_f64 * t26955 * t96999 - 0.46336805555555555556e-3_f64 * t26960 * t97083 * t15481 - 0.30918233506944444444e-4_f64 * t26955 * t96799 + 0.30891203703703703704e-3_f64 * t26960 * t97089 * t15256 - 0.46336805555555555556e-3_f64 * t26960 * t97093 * t15501 + 0.12897067901234567901e-2_f64 * t95946;
    (t97069, t97076, t97098)
}
