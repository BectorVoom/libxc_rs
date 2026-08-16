//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2647/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2647(t28: f64, t1081: f64, t5966: f64, t584: f64, t15952: f64, t15955: f64, t18196: f64, t19559: f64, t20385: f64, t20390: f64, t2219: f64, t3672: f64, t39436: f64, t5142: f64, t517: f64, t71090: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t73995 = t5966 * t1081;
    let t73998 = t584 * t5966;
    let t74009 = piecewise3(t29, 0.0_f64, 40.0_f64 / 81.0_f64 * t39436 * t20385 * t1081 + 16.0_f64 / 9.0_f64 * t19559 * t2219 - 8.0_f64 / 9.0_f64 * t15952 * t73995 - 8.0_f64 / 3.0_f64 * t15955 * t73998 + 4.0_f64 / 3.0_f64 * t5142 * t18196 + 4.0_f64 / 9.0_f64 * t3672 * t20390 * t1081 + 4.0_f64 / 3.0_f64 * t517 * t71090);
    (t73995, t73998, t74009)
}
