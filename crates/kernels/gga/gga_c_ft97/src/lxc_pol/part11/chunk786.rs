//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 786/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk786<F: Float>(t173: F, t419: F, t8089: F, t422: F, t7800: F, t37357: F, t420: F, t1742: F, t37362: F, t1744: F, t8130: F, t1725: F, t8126: F, t8109: F, t2248: F, t424: F) -> (F, F, F, F, F, F, F) {
    let t37763 = t419 * t173 * t8089;
    let t37765 = t422 * t7800;
    let t37768 = t419 * t420 * t37765 * t37357;
    let t37772 = t419 * t420 * t1742 * t37362;
    let t37774 = t8130 * t1744;
    let t37776 = t1725 * t8126;
    let t37778 = t1725 * t8109;
    let t37781 = t419 * t2248 * t424;
    (t37763, t37768, t37772, t37774, t37776, t37778, t37781)
}
