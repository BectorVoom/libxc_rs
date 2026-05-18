//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 511/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk511<F: Float>(t2279: F, t2281: F, t2312: F, t2315: F, t2317: F, t875: F, t158: F, t166: F, t104: F, t288: F, t1543: F, t97: F) -> (F, F, F, F) {
    let t2320 = -F::new(0.571528e-1) * t2279 + F::new(0.285764e-1) * t2281 + F::new(0.285764e-1) * t2312 * t875 - F::new(0.285764e-1) * t2315 * t2317;
    let t2321 = t2320 * t158;
    let t2322 = t2321 * t166;
    let t2323 = t104 * t288;
    let t2325 = t97 * t2323 * t1543;
    (t2320, t2321, t2322, t2325)
}
