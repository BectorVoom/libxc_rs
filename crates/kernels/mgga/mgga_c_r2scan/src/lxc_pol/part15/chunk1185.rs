//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1185/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1185<F: Float>(t40197: F, t40201: F, t40204: F, t40207: F, t40210: F, t40213: F, t40216: F, t40218: F, t40220: F, t40223: F, t40224: F, t40228: F) -> F {
    let t40230 = -F::new(0.32927245914677557994e0) * t40197 - F::new(0.95219938395347901943e-2) * t40201 + F::new(0.2600466522016280569e0) * t40204 - F::new(0.2600466522016280569e0) * t40207 + F::new(0.10975748638225852664e0) * t40210 - F::new(0.10401866088065122276e1) * t40213 - t40216 - t40218 + F::new(0.22511059664845582436e0) * t40220 - t40223 - F::new(0.43663693315433241792e-2) * t40224 + F::new(0.16262400898971305031e-3) * t40228;
    t40230
}
