//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 899/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk899(t1375: f64, t14267: f64, t16816: f64, t828: f64, t837: f64, t845: f64, t3788: f64, t4954: f64, t4958: f64, t10645: f64, t14029: f64, t1415: f64, t16931: f64, t16935: f64, t16941: f64, t16945: f64, t16947: f64, t3980: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16949 = 0.17544670192365612213e1_f64 * t14267 * t1375;
    let t16951 = t828 * t16816 * t837;
    let t16953 = 0.58482233974552040708e0_f64 * t845 * t16951;
    let t16955 = 0.17544670192365612213e1_f64 * t3788 * t4954;
    let t16957 = 0.51947267698127589899e2_f64 * t3788 * t4958;
    let t16958 = -t10645 / 3.0_f64 + t16931 - t16935 - 0.77534644304710291488e-2_f64 * t3980 * t14029 * t1415 - t16941 - t16945 + t16947 - t16949 - t16953 - t16955 - t16957;
    (t16949, t16951, t16953, t16955, t16957, t16958)
}
