//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 186/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk186<F: Float>(t608: F, t65: F, t34: F, t36: F, t43: F, t607: F, t55: F, t583: F, t61: F, rho0: F, sigma0: F) -> (F, F, F, F, F) {
    let t609 = t608 * t65;
    let t612 = t34 * rho0;
    let t614 = F::cast_from(1.0_f64) / t36 / t612;
    let t615 = sigma0 * t614;
    let t618 = t43 * t607;
    let t621 = t55 * t607;
    let t625 = F::cast_from(1.0_f64) / t61 / t583;
    (t609, t615, t618, t621, t625)
}
