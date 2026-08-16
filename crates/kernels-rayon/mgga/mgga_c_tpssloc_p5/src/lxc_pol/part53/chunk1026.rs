//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1026/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1026(t116498: f64, t123414: f64, t123418: f64, t123745: f64, t123752: f64, t16596: f64, t1877: f64, t24339: f64, t24344: f64, t2522: f64, t25365: f64, t25374: f64, t26739: f64, t32047: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t7114: f64, t7844: f64, t8748: f64) -> f64 {
    let t123835 = -6.0_f64 * t116498 * t1877 * t25374 - 6.0_f64 * t123414 * t2522 * t7114 + 4.0_f64 * t123418 * t1877 * t24344 - 6.0_f64 * t123745 * t2522 * t7114 + 4.0_f64 * t123752 * t1877 * t24344 + 6.0_f64 * t16596 * t2522 * t32047 - 2.0_f64 * t1877 * t24339 * t7844 - 2.0_f64 * t1877 * t26739 * t7114 + 2.0_f64 * t1877 * t32047 * t4303 + 6.0_f64 * t2522 * t25365 * t32047 - 3.0_f64 * t2522 * t4119 * t8748 - 6.0_f64 * t4255 * t4314 * t8748;
    t123835
}
