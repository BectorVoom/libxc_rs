//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1206/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1206(t413: f64, t15291: f64, t15468: f64, t1260: f64, t286: f64, t110: f64, t1852: f64, t1251: f64, t15255: f64, t4580: f64, t3515: f64, t330: f64, t421: f64, t992: f64) -> (f64, f64, f64, f64, f64) {
    let t418 = 0.0_f64 < t413;
    let t15469 = t15291 + t15468;
    let t15471 = piecewise3(t418, t15469, -t15469);
    let t15472 = t1260 * t15471;
    let t15473 = t286 * t15472;
    let t15476 = t110 * t1852;
    let t15477 = t1251 * t15476;
    let t15481 = t4580 * t15255;
    let t15482 = t3515 * t15481;
    let t15486 = t992 * t421 * t330;
    (t15469, t15473, t15477, t15482, t15486)
}
