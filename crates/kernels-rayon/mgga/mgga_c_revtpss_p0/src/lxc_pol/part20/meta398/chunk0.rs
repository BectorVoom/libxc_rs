//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1473/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1473(t11299: f64, t2918: f64, t2927: f64, t11380: f64, t2874: f64, t934: f64, t11379: f64, t2924: f64, t2926: f64, t11294: f64, t11531: f64, t41500: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t41864 = 0.57895126195293126241e3_f64 * t11299 * t2927 * t2918;
    let t41867 = 8.0_f64 * t2874 * t11380 * t934;
    let t41871 = 0.64327917994770140268e2_f64 * t2924 * t11379 * t2926 * t934;
    let t41873 = 24.0_f64 * t11294 * t11531;
    let t41876 = 24.0_f64 * t11299 * t41500 * t935;
    (t41864, t41867, t41871, t41873, t41876)
}
