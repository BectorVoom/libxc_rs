//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2176/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2176<F: Float>(t1888: F, t22996: F, t2632: F, t87106: F, t23143: F, t7525: F, t25238: F, t6579: F, t23153: F, t4119: F, t6552: F, t6637: F) -> (F, F, F, F) {
    let t87660 = t1888 * t22996 * t87106 * t2632;
    let t87666 = t23143 * t7525;
    let t87668 = t6579 * t25238;
    let t87669 = F::cast_from(0.38381794893125283518e-1_f64) * t87668;
    let t87672 = t6552 * t6637 * t23153 * t4119;
    (t87660, t87666, t87669, t87672)
}
