//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1203/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1203<F: Float>(t10017: F, t2131: F, t2147: F, t309: F, t40861: F, t7963: F, t9427: F, t38778: F, t7942: F, t33227: F, t33778: F, t38040: F, t38324: F, t38329: F, t38343: F, t38345: F, t38348: F, t39794: F, t40215: F, t40868: F, t556: F, t7931: F, t8400: F, t8791: F, t9165: F) -> F {
    let t41272 = t2131 * t2147 * t10017 * t309;
    let t41290 = t7963 * t9427 * t40861;
    let t41293 = t7942 * t9427 * t38778;
    let t41295 = t38324 - t33227 + F::new(0.17347256376410398924e1) * t38329 + F::new(0.17347256376410398924e1) * t41272 + F::new(0.34694512752820797848e1) * t7931 * t9427 * t556 * t8791 + F::new(0.26020884564615598386e1) * t8400 * t38040 * t40215 - F::new(0.26020884564615598386e1) * t8400 * t9427 * t39794 - F::new(0.17347256376410398924e1) * t33778 * t9165 - t38343 - t38345 + F::new(0.17347256376410398924e1) * t7931 * t9427 * t40868 - F::new(0.17347256376410398924e1) * t41290 + F::new(0.17347256376410398924e1) * t41293 - t38348;
    t41295
}
