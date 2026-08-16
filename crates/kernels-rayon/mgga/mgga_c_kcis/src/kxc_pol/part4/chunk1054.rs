//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1054/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1054(t1021: f64, t13314: f64, t1092: f64, t3220: f64, t4999: f64, t1747: f64, t3225: f64, t3229: f64, t1749: f64, t3237: f64, t303: f64, t4984: f64, t922: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13315 = t1021 * t13314;
    let t13316 = t1092 * t13315;
    let t13318 = t4999 * t3220;
    let t13319 = t1092 * t13318;
    let t13321 = t1747 * t3225;
    let t13322 = t13321 * sigma0;
    let t13323 = t13322 * t3229;
    let t13324 = t1092 * t13323;
    let t13326 = t1749 * t3237;
    let t13327 = t303 * t13326;
    let t13330 = t4984 * t922;
    (t13316, t13319, t13321, t13324, t13327, t13330)
}
