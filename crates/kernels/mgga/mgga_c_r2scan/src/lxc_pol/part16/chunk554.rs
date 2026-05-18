//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 554/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk554<F: Float>(t108: F, t3039: F, t3040: F, t1542: F, t2892: F, t3016: F, t490: F, t109: F, t111: F, t915: F, t917: F) -> (F, F, F, F) {
    let t3042 = (t3039 + t3040) * t108;
    let t3046 = t1542 * t2892;
    let t3049 = t490 * t3016;
    let t3052 = -F::new(12.0) * t109 * t3046 + F::new(3.0) * t109 * t3049 - t3042 * t111 + F::new(6.0) * t915 * t917;
    (t3042, t3046, t3049, t3052)
}
