//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 486/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk486<F: Float>(t1721: F, t61: F, t153: F, t158: F) -> (F, F, F, F) {
    let t1723 = 0.65061487801810439052e-1 * t61 * t1721;
    let t1724 = t153 * t153;
    let t1725 = 1.0 / t1724;
    let t1726 = t1725 * t158;
    (t1723, t1724, t1725, t1726)
}
