//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3203/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3203(t1238: f64, t12732: f64, t12866: f64, t12972: f64, t13029: f64, t13043: f64, t17283: f64, t17290: f64, t17514: f64, t17515: f64, t17756: f64, t3603: f64, t3663: f64, t3720: f64, t44561: f64, t44823: f64, t44829: f64, t44838: f64, t44884: f64, t5323: f64, t5327: f64, t5332: f64, t5340: f64, t5373: f64, t56981: f64, t58921: f64, t59011: f64, t59017: f64, t59025: f64, t59033: f64, t59041: f64, t59043: f64) -> f64 {
    let t59056 = 0.30011812682648815881e-2_f64 * t59011 * t3720 * t58921 * t13043 * t3603 + 0.64311027177104605458e-3_f64 * t59017 * t17756 + 0.42874018118069736972e-3_f64 * t5340 * t3720 * t5332 * t3603 * t12732 + 0.34299214494455789577e-2_f64 * t59025 * t1238 + 0.34299214494455789577e-2_f64 * t17283 * t3663 + 0.11433071498151929859e-2_f64 * t5323 * t12972 - 0.64311027177104605458e-3_f64 * t59033 * t1238 - 0.64311027177104605458e-3_f64 * t17290 * t3663 - 0.21437009059034868486e-3_f64 * t5327 * t12972 - t59041 - 0.85748036236139473944e-3_f64 * t59043 + 7.0_f64 / 243.0_f64 * t5373 * t13029 + 0.85748036236139473944e-3_f64 * t44561 * t17515 + 0.85748036236139473944e-3_f64 * t12866 * t56981 * t17514 + 0.14291339372689912324e-3_f64 * t44823 - 0.19055119163586549765e-3_f64 * t44829 - 0.28582678745379824648e-3_f64 * t44838 + 0.28582678745379824648e-3_f64 * t44884;
    t59056
}
