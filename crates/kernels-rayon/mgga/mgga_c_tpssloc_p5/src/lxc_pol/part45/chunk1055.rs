//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1055/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1055(t2105: f64, t7002: f64, t2098: f64, t7020: f64, t115972: f64, t115981: f64, t116011: f64, t116014: f64, t116021: f64, t116026: f64, t116028: f64, t116032: f64, t116036: f64, t1396: f64, t1398: f64, t1404: f64, t2023: f64, t2029: f64, t2099: f64, t23863: f64, t23901: f64, t24448: f64, t24486: f64, t3: f64, t31782: f64, t31820: f64, t3932: f64, t3946: f64, t580: f64, t7003: f64, t7223: f64, t7240: f64, t8647: f64, t8660: f64) -> f64 {
    let t116038 = t7002 * t2105;
    let t116044 = t2098 * t7020;
    let tv4rho2sigma21 = t1398 * (t115981 + t116011) + 2.0_f64 * t116014 + 2.0_f64 * t1396 * t31820 + t3 * t115972 * t580 + t2023 * t24486 + 2.0_f64 * t116021 + t23863 * t2105 + t8647 * t3946 + t2099 * t23901 + 2.0_f64 * t116026 + 2.0_f64 * t116028 + 2.0_f64 * t31782 * t1404 + 2.0_f64 * t116032 + t24448 * t2029 + t3932 * t8660 + 2.0_f64 * t116036 + 2.0_f64 * t116038 + 2.0_f64 * t7223 * t7020 + 2.0_f64 * t7003 * t7240 + 2.0_f64 * t116044;
    tv4rho2sigma21
}
