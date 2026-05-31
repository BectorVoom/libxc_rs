//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 634/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk634<F: Float>(t341: F, t3522: F, t1129: F, t1131: F, t1133: F, t1135: F, t1137: F, t343: F, t3524: F, t3526: F, t3530: F, t3534: F, t3538: F, t839: F) -> (F, F) {
    let t3542 = t341 * t3522;
    let t3548 = -F::cast_from(0.64e0_f64) * t3522 - F::cast_from(0.8704e0_f64) * t3524 - F::cast_from(0.8704e0_f64) * t3526 - F::cast_from(0.9214113627294e1_f64) * t1129 * t839 - F::cast_from(0.4607056813647e1_f64) * t3530 + F::cast_from(0.367387230261e2_f64) * t1131 * t839 + F::cast_from(0.122462410087e2_f64) * t3534 - F::cast_from(0.3831420472412e2_f64) * t1133 * t839 - F::cast_from(0.957855118103e1_f64) * t3538 + F::cast_from(0.1550653405116e2_f64) * t1135 * t839 + F::cast_from(0.3101306810232e1_f64) * t3542 - F::cast_from(0.2177652951264e1_f64) * t1137 * t839 - F::cast_from(0.362942158544e0_f64) * t343 * t3522;
    (t3542, t3548)
}
