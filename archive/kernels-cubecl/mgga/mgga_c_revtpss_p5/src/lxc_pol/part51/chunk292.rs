//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 292/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk292<F: Float>(t30: F, t33: F, t1312: F, t649: F, t670: F, t22: F, t583: F, t521: F, t19: F, t588: F, t513: F, t605: F, t1113: F, t516: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1315 = F::cast_from(2.0_f64) * t1312 * t670 + t649;
    let t1317 = t583 * t22;
    let t1319 = F::cast_from(4.0_f64) * t1317 * t521;
    let t1320 = t19 * t588;
    let t1322 = F::cast_from(4.0_f64) * t1320 * t521;
    let t1325 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t513 * t605);
    let t1328 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t516 * t1113);
    (t1315, t1317, t1319, t1320, t1322, t1325, t1328)
}
