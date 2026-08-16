//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1227/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1227(t413: f64, t20361: f64, t20549: f64, t1260: f64, t286: f64, t25: f64, t6838: f64, t1251: f64, t10990: f64, t15215: f64, t15219: f64, t15223: f64, t15477: f64, t15493: f64, t15496: f64, t20346: f64, t20350: f64, t3490: f64, t3514: f64, t6839: f64) -> (f64, f64) {
    let t418 = 0.0_f64 < t413;
    let t20550 = t20361 + t20549;
    let t20552 = piecewise3(t418, t20550, -t20550);
    let t20553 = t1260 * t20552;
    let t20554 = t286 * t20553;
    let t20559 = t25 * t6838;
    let t20560 = t1251 * t20559;
    let t20562 = -t15215 - t15219 + t15223 + t3514 * t20346 / 144.0_f64 - t3514 * t20350 / 216.0_f64 + t10990 / 864.0_f64 + t15477 / 432.0_f64 - t1251 * t20554 / 192.0_f64 + t3490 * t6839 / 72.0_f64 - t20560 / 576.0_f64 + t15493 - t15496;
    (t20550, t20562)
}
