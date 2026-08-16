//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2527/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2527<F: Float>(t51614: F, t10535: F, t14523: F, t9285: F, t10073: F, t14496: F, t14946: F, t2710: F, t14598: F, t14600: F, t2434: F, t836: F) -> (F, F, F, F, F) {
    let t51615 = F::cast_from(0.34697458558045176417e-2_f64) * t51614;
    let t51635 = t10535 * t14523 * t9285;
    let t51637 = t10073 * t14496;
    let t51646 = t2710 * t14946 * t9285;
    let t51657 = t14598 * t14600 * t2434 * t836;
    (t51615, t51635, t51637, t51646, t51657)
}
