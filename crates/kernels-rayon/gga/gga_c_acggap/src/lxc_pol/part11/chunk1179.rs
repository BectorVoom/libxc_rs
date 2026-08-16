//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1179/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1179(t1423: f64, t7746: f64, t31752: f64, t31751: f64, t36097: f64, t36100: f64, t36103: f64, t36107: f64, t36111: f64, t36115: f64, t36119: f64, t36123: f64, t36126: f64, t36127: f64, t36129: f64, t36132: f64, t36134: f64, t36135: f64, t36137: f64) -> f64 {
    let t36139 = t7746 * t1423;
    let t36141 = 0.26416397523267487738e-1_f64 * t31752;
    let t36142 = t36097 + 0.15724046144802076034e-3_f64 * t36100 + 0.10718504529517434243e-2_f64 * t36103 + 0.10718504529517434243e-2_f64 * t36107 + 0.53592522647587171215e-3_f64 * t36111 - 0.21437009059034868486e-3_f64 * t36115 + 0.41930789719472202757e-3_f64 * t36119 - 0.62896184579208304135e-3_f64 * t36123 + t36126 + 0.37737710747524982482e-2_f64 * t36127 - 0.21437009059034868486e-3_f64 * t36129 - t36132 - t36134 - 0.28582678745379824648e-3_f64 * t36135 + 0.19812298142450615803e-1_f64 * t36137 - 0.16006300097412701803e-1_f64 * t36139 - t31751 - t36141;
    t36142
}
