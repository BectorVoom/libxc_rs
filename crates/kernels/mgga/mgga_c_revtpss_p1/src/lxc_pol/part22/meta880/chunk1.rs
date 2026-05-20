//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3051/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3051<F: Float>(t10073: F, t14496: F, t231: F, t2782: F, t2783: F, t51625: F, t14946: F, t2710: F, t9285: F, t4469: F, t836: F, t14598: F, t14600: F, t2434: F) -> (F, F, F, F, F) {
    let t51637 = t10073 * t14496;
    let t51642 = t2782 * t2783 * t51625 * t231;
    let t51646 = t2710 * t14946 * t9285;
    let t51653 = t2782 * t2783 * t4469 * t836 * t231;
    let t51657 = t14598 * t14600 * t2434 * t836;
    (t51637, t51642, t51646, t51653, t51657)
}
