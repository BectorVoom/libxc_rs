//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1035/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1035<F: Float>(t7: F, t132: F, t10180: F, t10441: F, t10480: F, t10513: F, t9909: F, t4214: F, t849: F, t222: F, t4104: F, t568: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t10516 = piecewise3(t134, 0.0, t10180 + t10441 + t10480 + t10513);
    let t10517 = piecewise3(t8, 0.0, t9909);
    let t10528 = t4214 * t849;
    let t10534 = t222 * t568 * t4104;
    (t10516, t10517, t10528, t10534)
}
