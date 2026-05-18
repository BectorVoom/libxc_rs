//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 452/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk452<F: Float>(t481: F, t788: F, t2207: F, t785: F, t1604: F, t2158: F, t110: F, t57: F) -> (F, F, F, F) {
    let t2208 = t788 * t481;
    let t2210 = t2207 * t785 * t2208;
    let t2212 = t1604 * t2158;
    let t2214 = t57 * t110;
    (t2208, t2210, t2212, t2214)
}
