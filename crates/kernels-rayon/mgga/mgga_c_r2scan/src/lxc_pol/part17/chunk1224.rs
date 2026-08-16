//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1224/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1224(t43144: f64, t43146: f64, t43149: f64, t43151: f64, t43153: f64, t43155: f64, t43157: f64, t43160: f64, t43162: f64, t43165: f64, t43167: f64, t43169: f64) -> f64 {
    let t44255 = 0.21951497276451705328e0_f64 * t43144 - 0.32927245914677557992e0_f64 * t43146 - 0.52009330440325611378e0_f64 * t43149 + 0.17336443480108537126e0_f64 * t43151 + 0.10975748638225852664e0_f64 * t43153 - 0.54878743191129263322e-1_f64 * t43155 - 0.32927245914677557992e0_f64 * t43157 + 0.17336443480108537126e0_f64 * t43160 + 0.21951497276451705328e0_f64 * t43162 - 0.17336443480108537126e0_f64 * t43165 + 0.52009330440325611378e0_f64 * t43167 - 0.23115257973478049502e0_f64 * t43169;
    t44255
}
