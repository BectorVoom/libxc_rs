//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1183/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1183(t1851: f64, t3611: f64, t3643: f64, t5272: f64, t11182: f64, t1844: f64, t1281: f64, t15690: f64, t11229: f64, t1864: f64, t3668: f64, t5358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47050 = t1851 * t3611;
    let t47323 = t5272 * t3643;
    let t47652 = t1844 * t11182;
    let t47681 = t15690 * t1281;
    let t47700 = t1864 * t11229;
    let t47711 = t5358 * t3668;
    (t47050, t47323, t47652, t47681, t47700, t47711)
}
