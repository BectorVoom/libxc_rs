//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3123/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3123(t18287: f64, t225: f64, t11925: f64, t11928: f64, t1235: f64, t1252: f64, t14980: f64, t15771: f64, t15789: f64, t15790: f64, t15797: f64, t15803: f64, t1720: f64, t1761: f64, t18571: f64, t19209: f64, t19249: f64, t27784: f64, t3590: f64, t3593: f64, t3600: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t53677: f64, t53703: f64, t6150: f64, t6244: f64, t6268: f64) -> f64 {
    let t64595 = t18287 * t225;
    let t64602 = 2.0_f64 * t1235 * t18571 * t498 + 2.0_f64 * t15771 * t1720 * t498 - 24.0_f64 * t15789 * t27784 * t53677 + t3590 * t498 * t6150 + 2.0_f64 * t11925 * t6244 + 2.0_f64 * t11928 * t6244 - t11928 * t6268 - 2.0_f64 * t1252 * t64595 - 4.0_f64 * t14980 * t5089 + 8.0_f64 * t15790 * t4945 + 8.0_f64 * t15790 * t5055 - 4.0_f64 * t15797 * t5089 + 4.0_f64 * t15803 * t4945 - 4.0_f64 * t1761 * t53703 - 2.0_f64 * t19209 * t3593 + 2.0_f64 * t19249 * t3600;
    t64602
}
