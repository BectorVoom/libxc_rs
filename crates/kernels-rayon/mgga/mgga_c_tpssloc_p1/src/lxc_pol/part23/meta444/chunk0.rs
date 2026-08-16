//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1288/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1288(t12211: f64, t20516: f64, t20501: f64, t3726: f64, t54042: f64, t6390: f64, t20479: f64, t3866: f64, t16336: f64, t6427: f64, t1824: f64, t6414: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74393 = t12211 * t20516;
    let t74395 = t3726 * t20501;
    let t74401 = t54042 * t6390;
    let t74403 = t3866 * t20479;
    let t74405 = t16336 * t6427;
    let t74415 = t6414 * t1824;
    (t74393, t74395, t74401, t74403, t74405, t74415)
}
