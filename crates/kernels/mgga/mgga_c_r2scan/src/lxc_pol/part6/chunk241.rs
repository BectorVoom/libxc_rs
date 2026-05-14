//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 241/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk241<F: Float>(t182: F, t377: F, t190: F, t625: F, t175: F, t181: F) -> (F, F, F, F) {
    let t626 = t377 * t182;
    let t629 = 0.17808333333333333333e-1 * t625 * t626 * t190;
    let t630 = t181 * t175;
    let t631 = 1.0 / t630;
    (t626, t629, t630, t631)
}
