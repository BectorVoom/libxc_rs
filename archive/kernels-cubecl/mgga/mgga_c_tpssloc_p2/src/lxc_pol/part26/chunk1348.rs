//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1348/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1348<F: Float>(t11605: F, t225: F, t1184: F, t3470: F, t3597: F, t3599: F, t2122: F, t7303: F, t85660: F, t3590: F, t7299: F, t24571: F, t24574: F) -> (F, F, F, F, F, F, F) {
    let t85674 = t225 * t11605;
    let t85683 = t3470 * t1184;
    let t85687 = t3597 * t3599;
    let t85688 = t2122 * t85687;
    let t85701 = t85660 * t7303;
    let t85707 = t7299 * t3590;
    let t85711 = t24574 * t24571;
    (t85674, t85683, t85687, t85688, t85701, t85707, t85711)
}
