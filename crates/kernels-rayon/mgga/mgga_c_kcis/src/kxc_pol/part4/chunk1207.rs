//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1207/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1207(t15486: f64, t737: f64, t3490: f64, t5321: f64, t25: f64, t5337: f64, t1251: f64, t10990: f64, t10993: f64, t10996: f64, t11009: f64, t11014: f64, t11086: f64, t15473: f64, t15477: f64, t15482: f64, t1853: f64, t3514: f64, t5307: f64, t5311: f64, t5338: f64) -> f64 {
    let t15487 = t737 * t15486;
    let t15493 = t3490 * t5321 / 108.0_f64;
    let t15494 = t25 * t5337;
    let t15496 = t1251 * t15494 / 288.0_f64;
    let t15499 = t11086 * t5307 / 108.0_f64 + t11086 * t5311 / 54.0_f64 + t10990 / 432.0_f64 - t10993 / 576.0_f64 - t1251 * t15473 / 192.0_f64 + t15477 / 864.0_f64 + t3490 * t5338 / 36.0_f64 + t3514 * t15482 / 144.0_f64 + t1251 * t15487 / 288.0_f64 - 11.0_f64 / 216.0_f64 * t10996 * t1853 + t15493 - t15496 + t11009 / 108.0_f64 + t11014 / 288.0_f64;
    t15499
}
