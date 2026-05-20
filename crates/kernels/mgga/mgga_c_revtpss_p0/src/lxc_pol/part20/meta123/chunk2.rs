//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 714/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk714<F: Float>(t1150: F, t3385: F, t3384: F, t406: F, t409: F, t1134: F) -> (F, F, F, F) {
    let t3386 = t3385 * t1150;
    let t3388 = F::new(2.0) * t3384 * t3386;
    let t3390 = F::new(1.0) / t409 / t406;
    let t3391 = t1134 * t1134;
    (t3386, t3388, t3390, t3391)
}
