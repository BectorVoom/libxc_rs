//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 650/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk650(t341: f64, t3745: f64, t1020: f64, t1129: f64, t1131: f64, t1133: f64, t1135: f64, t1137: f64, t343: f64, t3747: f64, t3749: f64, t3753: f64, t3757: f64, t3761: f64) -> (f64, f64) {
    let t3765 = t341 * t3745;
    let t3771 = -0.64e0_f64 * t3745 - 0.8704e0_f64 * t3747 - 0.8704e0_f64 * t3749 - 0.9214113627294e1_f64 * t1129 * t1020 - 0.4607056813647e1_f64 * t3753 + 0.367387230261e2_f64 * t1131 * t1020 + 0.122462410087e2_f64 * t3757 - 0.3831420472412e2_f64 * t1133 * t1020 - 0.957855118103e1_f64 * t3761 + 0.1550653405116e2_f64 * t1135 * t1020 + 0.3101306810232e1_f64 * t3765 - 0.2177652951264e1_f64 * t1137 * t1020 - 0.362942158544e0_f64 * t343 * t3745;
    (t3765, t3771)
}
