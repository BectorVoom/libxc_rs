//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2677/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2677(t16398: f64, t20470: f64, t12419: f64, t1307: f64, t16242: f64, t20448: f64, t20500: f64, t210: f64, t3733: f64, t3803: f64, t54132: f64, t54151: f64, t56837: f64, t56853: f64, t56883: f64, t56885: f64, t56888: f64, t56906: f64, t56909: f64, t56919: f64, t56921: f64) -> f64 {
    let t74618 = t16398 * t20470;
    let t74632 = -5.0_f64 / 256.0_f64 * t3803 * t12419 * t16242 * t20448 + 7.0_f64 / 1536.0_f64 * t56837 - 7.0_f64 / 192.0_f64 * t56853 + 7.0_f64 / 192.0_f64 * t74618 + t54132 + 7.0_f64 / 256.0_f64 * t56883 - 7.0_f64 / 256.0_f64 * t56885 - 7.0_f64 / 384.0_f64 * t56888 - 7.0_f64 / 192.0_f64 * t56906 + t3733 * t210 * t20500 * t1307 / 16.0_f64 + 595.0_f64 / 3456.0_f64 * t54151 - 35.0_f64 / 192.0_f64 * t56909 + 7.0_f64 / 768.0_f64 * t56919 + 7.0_f64 / 768.0_f64 * t56921;
    t74632
}
