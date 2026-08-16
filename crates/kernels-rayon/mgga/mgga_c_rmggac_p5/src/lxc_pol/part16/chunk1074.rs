//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1074/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1074(t39048: f64, t42800: f64, t42806: f64, t42820: f64, t42821: f64, t42823: f64, t45120: f64, t45123: f64, t45126: f64, t45129: f64, t45132: f64, t45135: f64, t45139: f64, t45149: f64, t45152: f64, t45155: f64, t45158: f64) -> f64 {
    let t48357 = -t42800 - t42806 + t42820 - t42821 - t42823 + 0.16364796992547205038e0_f64 * t45120 - 0.2727466165424534173e0_f64 * t45123 - 0.10909864661698136692e0_f64 * t45126 - 0.81823984962736025192e-1_f64 * t45129 + 0.16364796992547205038e0_f64 * t45132 + 0.81823984962736025192e-1_f64 * t45135 + 0.40911992481368012596e-1_f64 * t45139 - 0.18183107769496894487e-1_f64 * t45149 + 0.1440846329149835838e-2_f64 * t45152 - 0.20496175532535769482e-3_f64 * t45155 + 0.1454648621559751559e0_f64 * t39048 - 0.5454932330849068346e-1_f64 * t45158;
    t48357
}
