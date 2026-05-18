//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1254/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1254<F: Float>(t37532: F, t37542: F, t37556: F, t37561: F, t37564: F, t37569: F, t40513: F, t40515: F, t40519: F, t43760: F, t43764: F, t43766: F, t43770: F, t43774: F, t43778: F) -> F {
    let t43914 = t37532 + t43760 + t43764 - t43766 - t43770 - t43774 - t37542 - F::new(0.30487649791575028314e-3) * t40513 + F::new(0.30487649791575028314e-3) * t40515 - t40519 + F::new(0.81300399444200075504e-3) * t37556 + t37561 - F::new(0.15243824895787514157e-3) * t37564 - t37569 + t43778;
    t43914
}
