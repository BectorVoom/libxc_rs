//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1224/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1224<F: Float>(t43144: F, t43146: F, t43149: F, t43151: F, t43153: F, t43155: F, t43157: F, t43160: F, t43162: F, t43165: F, t43167: F, t43169: F) -> F {
    let t44255 = F::cast_from(0.21951497276451705328e0_f64) * t43144 - F::cast_from(0.32927245914677557992e0_f64) * t43146 - F::cast_from(0.52009330440325611378e0_f64) * t43149 + F::cast_from(0.17336443480108537126e0_f64) * t43151 + F::cast_from(0.10975748638225852664e0_f64) * t43153 - F::cast_from(0.54878743191129263322e-1_f64) * t43155 - F::cast_from(0.32927245914677557992e0_f64) * t43157 + F::cast_from(0.17336443480108537126e0_f64) * t43160 + F::cast_from(0.21951497276451705328e0_f64) * t43162 - F::cast_from(0.17336443480108537126e0_f64) * t43165 + F::cast_from(0.52009330440325611378e0_f64) * t43167 - F::cast_from(0.23115257973478049502e0_f64) * t43169;
    t44255
}
