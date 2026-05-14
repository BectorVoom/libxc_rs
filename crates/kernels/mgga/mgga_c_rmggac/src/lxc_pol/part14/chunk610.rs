//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 610/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk610<F: Float>(t1990: F, t2412: F, t457: F, t589: F, t201: F, t1979: F, t1982: F, t2186: F, t2310: F, t2289: F, t2286: F, t2283: F, t1614: F, t36: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8685 = t2412 * t1990;
    let t8687 = t589 * t457;
    let t8688 = t8687 * t201;
    let t8690 = t8688 * t1979 * t1982;
    let t8692 = t2186 * t2310;
    let t8694 = t2186 * t2289;
    let t8696 = t2186 * t2286;
    let t8698 = t2186 * t2283;
    let t8700 = t36 * t1614;
    (t8685, t8687, t8688, t8690, t8692, t8694, t8696, t8698, t8700)
}
