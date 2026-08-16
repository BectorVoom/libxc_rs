//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1251/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1251<F: Float>(t24348: F, t24361: F, t1169: F, t12472: F, t24330: F, t1756: F, t6518: F) -> (F, F, F, F) {
    let t24362 = t24348 + t24361;
    let t24363 = t24362 * t1169;
    let t24366 = t24330 * t12472;
    let t24375 = t6518 * t1756;
    (t24362, t24363, t24366, t24375)
}
