//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 933/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk933<F: Float>(t39048: F, t42800: F, t42806: F, t42820: F, t42821: F, t42823: F, t45120: F, t45123: F, t45126: F, t45129: F, t45132: F, t45135: F, t45139: F, t45149: F, t45152: F, t45155: F, t45158: F) -> (F,) {
    let t48357 = -t42800 - t42806 + t42820 - t42821 - t42823 + 0.16364796992547205038e0 * t45120 - 0.2727466165424534173e0 * t45123 - 0.10909864661698136692e0 * t45126 - 0.81823984962736025192e-1 * t45129 + 0.16364796992547205038e0 * t45132 + 0.81823984962736025192e-1 * t45135 + 0.40911992481368012596e-1 * t45139 - 0.18183107769496894487e-1 * t45149 + 0.1440846329149835838e-2 * t45152 - 0.20496175532535769482e-3 * t45155 + 0.1454648621559751559e0 * t39048 - 0.5454932330849068346e-1 * t45158;
    (t48357,)
}
