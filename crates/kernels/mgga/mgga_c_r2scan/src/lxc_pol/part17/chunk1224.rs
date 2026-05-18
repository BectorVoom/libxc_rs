//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1224/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1224<F: Float>(t43144: F, t43146: F, t43149: F, t43151: F, t43153: F, t43155: F, t43157: F, t43160: F, t43162: F, t43165: F, t43167: F, t43169: F) -> F {
    let t44255 = F::new(0.21951497276451705328e0) * t43144 - F::new(0.32927245914677557992e0) * t43146 - F::new(0.52009330440325611378e0) * t43149 + F::new(0.17336443480108537126e0) * t43151 + F::new(0.10975748638225852664e0) * t43153 - F::new(0.54878743191129263322e-1) * t43155 - F::new(0.32927245914677557992e0) * t43157 + F::new(0.17336443480108537126e0) * t43160 + F::new(0.21951497276451705328e0) * t43162 - F::new(0.17336443480108537126e0) * t43165 + F::new(0.52009330440325611378e0) * t43167 - F::new(0.23115257973478049502e0) * t43169;
    t44255
}
