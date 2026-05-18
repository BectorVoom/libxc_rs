//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1280/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1280<F: Float>(t37556: F, t37564: F, t39097: F, t39099: F, t40515: F, t42229: F, t44915: F, t44918: F, t44921: F, t44926: F, t44928: F, t44931: F, t44933: F, t44935: F, t44937: F) -> F {
    let t45015 = F::new(0.60975299583150056624e-3) * t40515 - t42229 - t44915 + t44918 + F::new(0.162600798888400151e-2) * t37556 + t39097 - t44921 - F::new(0.30487649791575028312e-3) * t37564 - t39099 + t44926 + t44928 + t44931 + t44933 - t44935 + t44937;
    t45015
}
