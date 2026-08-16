//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1071/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1071(t1014: f64, t7928: f64, t27348: f64, t7898: f64, t1458: f64, t1466: f64, t2244: f64, t3245: f64, t110: f64, t2238: f64, t2237: f64, t1505: f64, t7938: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27462 = t1014 * t7928;
    let t27471 = t7898 * t27348;
    let t27475 = t1458 * t1466;
    let t27482 = t3245 * t2244;
    let t27483 = 0.55273148148148148147e-3_f64 * t27482;
    let t27484 = t110 * t2238;
    let t27486 = 0.15445601851851851852e-3_f64 * t2237 * t27484;
    let t27491 = t7938 * t1505;
    (t27462, t27471, t27475, t27482, t27483, t27484, t27486, t27491)
}
