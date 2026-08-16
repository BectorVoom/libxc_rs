//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1830/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1830<F: Float>(t22470: F, t4067: F, t1453: F, t2332: F, t81446: F, t666: F, t22473: F, t2358: F, t12808: F, t6530: F, t12816: F, t191: F, t192: F) -> (F, F, F, F, F, F) {
    let t86590 = t22470 * t4067;
    let t86592 = t1453 * t2332;
    let t86593 = t81446 * t86592;
    let t86595 = t4067 * t666;
    let t86596 = t22473 * t86595;
    let t86598 = t1453 * t2358;
    let t86599 = t22473 * t86598;
    let t86601 = t6530 * t12808;
    let t86672 = t12816 * t191 * t192;
    (t86590, t86593, t86596, t86599, t86601, t86672)
}
