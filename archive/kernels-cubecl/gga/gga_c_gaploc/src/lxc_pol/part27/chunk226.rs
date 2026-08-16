//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 226/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk226<F: Float>(t723: F, t808: F, t568: F, t325: F, t579: F, t61: F, t120: F, t320: F) -> (F, F, F, F, F) {
    let t814 = t808 * t723;
    let t815 = t568 * t814;
    let t818 = t579 * t325;
    let t819 = t61 * t818;
    let t822 = t320 * t120;
    (t814, t815, t818, t819, t822)
}
