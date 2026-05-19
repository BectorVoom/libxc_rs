//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 600/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk600<F: Float>(t2508: F, t3448: F, t2969: F, t977: F, t1052: F, t2592: F) -> (F, F, F, F) {
    let t3450 = F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t3448;
    let t3457 = t2969 * t977;
    let t3458 = t2592 * t1052;
    let t3459 = t1052 * t977;
    (t3450, t3457, t3458, t3459)
}
