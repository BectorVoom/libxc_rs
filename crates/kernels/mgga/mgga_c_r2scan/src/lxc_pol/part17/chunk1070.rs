//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1070/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1070<F: Float>(t39558: F, t41435: F, t43115: F, t43117: F, t43120: F, t43123: F, t43126: F, t43130: F, t43133: F, t43135: F, t43138: F, t43141: F, t43144: F, t43146: F, t43149: F, t43151: F, t43153: F, t43155: F, t43157: F, t43160: F, t43162: F, t43165: F, t43167: F, t43169: F) -> (F, F) {
    let t44242 = 0.10975748638225852664e0 * t43115 - t41435 - 0.10975748638225852664e-1 * t43117 - 0.90044238659382329742e0 * t39558 - 0.26198215989259945077e-1 * t43120 + 0.87327386630866483588e-2 * t43123 + 0.43663693315433241794e-2 * t43126 + 0.43663693315433241794e-2 * t43130 - 0.27944763721877274748e0 * t43133 + 0.5200933044032561138e0 * t43135 + 0.2600466522016280569e0 * t43138 + 0.52009330440325611378e0 * t43141;
    let t44255 = 0.21951497276451705328e0 * t43144 - 0.32927245914677557992e0 * t43146 - 0.52009330440325611378e0 * t43149 + 0.17336443480108537126e0 * t43151 + 0.10975748638225852664e0 * t43153 - 0.54878743191129263322e-1 * t43155 - 0.32927245914677557992e0 * t43157 + 0.17336443480108537126e0 * t43160 + 0.21951497276451705328e0 * t43162 - 0.17336443480108537126e0 * t43165 + 0.52009330440325611378e0 * t43167 - 0.23115257973478049502e0 * t43169;
    (t44242, t44255)
}
