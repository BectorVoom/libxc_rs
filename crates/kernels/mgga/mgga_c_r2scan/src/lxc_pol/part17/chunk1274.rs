//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1274/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1274<F: Float>(t37419: F, t37423: F, t40303: F, t40305: F, t40308: F, t40315: F, t42187: F, t43854: F, t44140: F, t44143: F, t44147: F, t44150: F, t44152: F, t44155: F, t44158: F) -> F {
    let t44979 = F::new(0.29810146462873361016e-2) * t37419 + t44140 - t44143 + F::new(0.72042316457491791901e-3) * t37423 - t44147 + t44150 - t44152 - t44155 + F::new(0.30487649791575028312e-3) * t43854 - t44158 - F::new(0.7684513755465791136e-2) * t40303 + F::new(0.18446557979282192534e-2) * t40305 + F::new(0.1440846329149835838e-2) * t40308 + t42187 - F::new(0.17347588262831798123e-3) * t40315;
    t44979
}
