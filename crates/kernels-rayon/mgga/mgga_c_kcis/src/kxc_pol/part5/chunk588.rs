//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 588/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk588(t109: f64, t287: f64, t209: f64, t421: f64, t416: f64, t25: f64, t992: f64, t1254: f64, t1251: f64, t1263: f64, t286: f64, t2887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3495 = t109 * t287;
    let t3497 = t209 * t3495 * t421;
    let t3499 = t416 * t3497 / 864.0_f64;
    let t3500 = t25 * t992;
    let t3501 = t3500 * t1254;
    let t3502 = t1251 * t3501;
    let t3504 = t25 * t1263;
    let t3505 = t1251 * t3504;
    let t3507 = t286 * t2887;
    (t3497, t3499, t3500, t3502, t3505, t3507)
}
