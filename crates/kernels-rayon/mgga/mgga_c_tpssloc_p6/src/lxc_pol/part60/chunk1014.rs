//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1014/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1014(t101226: f64, t115027: f64, t121782: f64, t126176: f64, t126197: f64, t128075: f64, t128086: f64, t128097: f64, t128101: f64, t128110: f64, t1484: f64, t1530: f64, t1877: f64, t1914: f64, t193: f64, t202: f64, t23295: f64, t24191: f64, t24344: f64, t2522: f64, t26744: f64, t28248: f64, t28447: f64, t31434: f64, t33466: f64, t33476: f64, t33483: f64, t4314: f64, t5527: f64, t5544: f64, t5660: f64, t5664: f64, t7114: f64, t7540: f64, t84766: f64, t8566: f64, t870: f64, t93000: f64) -> f64 {
    let t128193 = t193 * t202 * t128075 * t870 + 6.0_f64 * t2522 * t33466 * t1484 + 3.0_f64 * t2522 * t8566 * t5544 - 2.0_f64 * t1877 * t121782 * t1530 - 6.0_f64 * t2522 * t7114 * t126176 + 4.0_f64 * t1877 * t24344 * t126197 - 3.0_f64 * t2522 * t7114 * t128086 - 2.0_f64 * t1877 * t26744 * t7540 - 6.0_f64 * t1877 * t84766 * t128110 + 12.0_f64 * t24191 * t23295 * t28248 + 6.0_f64 * t4314 * t8566 * t5527 - t1877 * t101226 * t1914 - 6.0_f64 * t2522 * t26744 * t33476 + 2.0_f64 * t1877 * t24344 * t128101 + 4.0_f64 * t1877 * t93000 * t33483 - 6.0_f64 * t2522 * t31434 * t28248 - 6.0_f64 * t4314 * t7114 * t128097 - t1877 * t31434 * t5660 + 2.0_f64 * t1877 * t115027 * t5664 - t1877 * t7114 * t28447;
    t128193
}
