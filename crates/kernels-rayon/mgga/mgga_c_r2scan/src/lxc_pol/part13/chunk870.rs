//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 870/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk870(t4842: f64, t4845: f64, t5020: f64, t6010: f64, t6012: f64, t7020: f64, t7021: f64, t7025: f64, t7031: f64, t7033: f64, t7036: f64, t6026: f64, t7052: f64, t7055: f64, t7093: f64, t7095: f64, t7097: f64, t7098: f64, t7101: f64, t7104: f64, t7133: f64, t7136: f64, t765: f64) -> (f64, f64) {
    let t7884 = t7020 - t7021 + t5020 + t6010 - 0.1143056e0_f64 * t6012 - t4842 - t7025 - t7031 - t7033 + t7036 + t4845;
    let t7895 = 0.675260332e-1_f64 * t765 * t7098 + 0.1350520664e0_f64 * t765 * t7101 + 0.675260332e-1_f64 * t765 * t7104 + 0.675260332e-1_f64 * t765 * t7133 + 0.1350520664e0_f64 * t765 * t7136 + t7052 - t7055 - t6026 - t7093 - t7095 + t7097;
    (t7884, t7895)
}
