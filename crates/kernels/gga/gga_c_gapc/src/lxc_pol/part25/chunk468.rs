//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 468/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk468<F: Float>(t2786: F, t285: F, t191: F, t1775: F, t315: F, t331: F) -> (F, F, F, F) {
    let t2787 = t2786 * t285;
    let t2788 = t2787 * t191;
    let t2795 = t1775 * t315;
    let t2800 = t331 * t331;
    let t2801 = 1.0 / t2800;
    (t2787, t2788, t2795, t2801)
}
