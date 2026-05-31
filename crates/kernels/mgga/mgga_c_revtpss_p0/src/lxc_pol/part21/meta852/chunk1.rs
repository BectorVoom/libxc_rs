//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3203/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3203<F: Float>(t1238: F, t12732: F, t12866: F, t12972: F, t13029: F, t13043: F, t17283: F, t17290: F, t17514: F, t17515: F, t17756: F, t3603: F, t3663: F, t3720: F, t44561: F, t44823: F, t44829: F, t44838: F, t44884: F, t5323: F, t5327: F, t5332: F, t5340: F, t5373: F, t56981: F, t58921: F, t59011: F, t59017: F, t59025: F, t59033: F, t59041: F, t59043: F) -> F {
    let t59056 = F::cast_from(0.30011812682648815881e-2_f64) * t59011 * t3720 * t58921 * t13043 * t3603 + F::cast_from(0.64311027177104605458e-3_f64) * t59017 * t17756 + F::cast_from(0.42874018118069736972e-3_f64) * t5340 * t3720 * t5332 * t3603 * t12732 + F::cast_from(0.34299214494455789577e-2_f64) * t59025 * t1238 + F::cast_from(0.34299214494455789577e-2_f64) * t17283 * t3663 + F::cast_from(0.11433071498151929859e-2_f64) * t5323 * t12972 - F::cast_from(0.64311027177104605458e-3_f64) * t59033 * t1238 - F::cast_from(0.64311027177104605458e-3_f64) * t17290 * t3663 - F::cast_from(0.21437009059034868486e-3_f64) * t5327 * t12972 - t59041 - F::cast_from(0.85748036236139473944e-3_f64) * t59043 + F::cast_from(7.0_f64) / F::cast_from(243.0_f64) * t5373 * t13029 + F::cast_from(0.85748036236139473944e-3_f64) * t44561 * t17515 + F::cast_from(0.85748036236139473944e-3_f64) * t12866 * t56981 * t17514 + F::cast_from(0.14291339372689912324e-3_f64) * t44823 - F::cast_from(0.19055119163586549765e-3_f64) * t44829 - F::cast_from(0.28582678745379824648e-3_f64) * t44838 + F::cast_from(0.28582678745379824648e-3_f64) * t44884;
    t59056
}
