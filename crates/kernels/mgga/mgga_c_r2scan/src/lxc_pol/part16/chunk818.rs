//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 818/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk818<F: Float>(t44: F, t1048: F, t795: F, t8601: F, t2999: F, t4904: F, t1212: F, t3002: F, t472: F, t8571: F, t1217: F, t2509: F, t415: F, zeta_threshold: F) -> (F, F) {
    let t45 = t44 <= zeta_threshold;
    let t8603 = t1048 * t8601 * t795;
    let t8604 = t4904 * t2999;
    let t8609 = t1212 * t3002;
    let t8612 = t472 * t8571;
    let t8615 = piecewise3::<F>(t45, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t8604 * t415 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2509 * t1217 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8609 * t415 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t8612);
    (t8603, t8615)
}
