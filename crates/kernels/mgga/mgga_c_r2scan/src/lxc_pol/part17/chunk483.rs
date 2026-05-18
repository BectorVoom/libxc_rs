//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 483/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk483<F: Float>(t2526: F, t490: F, t109: F, t111: F, t2498: F, t2504: F, t2506: F, t486: F, t491: F, t915: F, t917: F) -> (F, F) {
    let t2527 = t490 * t2526;
    let t2530 = F::new(3.0) * t109 * t2527 - t2498 * t111 - F::new(12.0) * t2504 * t2506 + F::new(3.0) * t486 * t917 + F::new(3.0) * t915 * t491;
    (t2527, t2530)
}
