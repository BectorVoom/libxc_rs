//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1376/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1376<F: Float>(t33482: F, t33464: F, t33474: F, t36596: F, t36597: F, t36599: F, t36600: F, t36601: F, t36602: F, t36604: F, t36605: F, t33487: F) -> (F, F) {
    let t36606 = F::new(0.77294542590142724634e-6) * t33482;
    let t36607 = -t36596 - t36597 - F::new(0.18115908419564701085e-6) * t33464 + t36599 - t36600 + t36601 + t36602 - F::new(0.5691280480400994668e-7) * t33474 - t36604 + t36605 + t36606;
    let t36609 = F::new(0.1374296967252737644e-5) * t33487;
    (t36607, t36609)
}
