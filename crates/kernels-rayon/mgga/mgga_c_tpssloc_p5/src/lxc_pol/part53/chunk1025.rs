//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1025/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1025(t116476: f64, t116492: f64, t123714: f64, t123719: f64, t1484: f64, t1530: f64, t16596: f64, t1877: f64, t193: f64, t202: f64, t2522: f64, t25365: f64, t25374: f64, t32030: f64, t32034: f64, t33991: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t776: f64, t868: f64, t870: f64, t8744: f64) -> f64 {
    let t123798 = t123714 * t193 * t202 * t870 - t116476 * t1530 * t1877 + 2.0_f64 * t116492 * t1877 * t25374 - t123719 * t1877 * t868 + 3.0_f64 * t1484 * t2522 * t32030 - 3.0_f64 * t16596 * t2522 * t32034 - t1877 * t32034 * t4303 - 3.0_f64 * t2522 * t25365 * t32034 + 3.0_f64 * t2522 * t33991 * t776 + 3.0_f64 * t2522 * t4119 * t8744 + 6.0_f64 * t4255 * t4314 * t8744;
    t123798
}
