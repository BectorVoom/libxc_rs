//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 821/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk821<F: Float>(t38659: F, t8419: F, t8466: F, t8501: F, t1841: F, t1851: F, t1853: F, t8213: F, t8392: F, t8520: F, t1570: F, t1580: F, t1557: F, t1882: F, t8570: F, t8529: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38660 = t38659 * t8419;
    let t38662 = t8466 * t8501;
    let t38664 = t1841 * t1851;
    let t38665 = t38664 * t1853;
    let t38675 = t8392 * t8213;
    let t38681 = t8392 * t8520;
    let t38688 = t1570 * t1580;
    let t38693 = t1557 * t1580;
    let t38698 = t1882 * t8570;
    let t38700 = t1882 * t8529;
    (t38660, t38662, t38665, t38675, t38681, t38688, t38693, t38698, t38700)
}
