//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1476/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1476<F: Float>(t120719: F, t120721: F, t120728: F, t120730: F, t120735: F, t122697: F, t122700: F, t122706: F, t122708: F, t122710: F, t122713: F, t122758: F, t1393: F, t33746: F, t34146: F, t7220: F) -> F {
    let t125020 = t1393 * t34146 - t33746 * t7220 - t120719 - t120721 - t120728 - t120730 - t120735 - t122697 + t122700 - t122706 - t122708 - t122710 - t122713 + t122758;
    t125020
}
