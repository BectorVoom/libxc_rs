//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 634/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk634(t341: f64, t3522: f64, t1129: f64, t1131: f64, t1133: f64, t1135: f64, t1137: f64, t343: f64, t3524: f64, t3526: f64, t3530: f64, t3534: f64, t3538: f64, t839: f64) -> (f64, f64) {
    let t3542 = t341 * t3522;
    let t3548 = -0.64e0_f64 * t3522 - 0.8704e0_f64 * t3524 - 0.8704e0_f64 * t3526 - 0.9214113627294e1_f64 * t1129 * t839 - 0.4607056813647e1_f64 * t3530 + 0.367387230261e2_f64 * t1131 * t839 + 0.122462410087e2_f64 * t3534 - 0.3831420472412e2_f64 * t1133 * t839 - 0.957855118103e1_f64 * t3538 + 0.1550653405116e2_f64 * t1135 * t839 + 0.3101306810232e1_f64 * t3542 - 0.2177652951264e1_f64 * t1137 * t839 - 0.362942158544e0_f64 * t343 * t3522;
    (t3542, t3548)
}
