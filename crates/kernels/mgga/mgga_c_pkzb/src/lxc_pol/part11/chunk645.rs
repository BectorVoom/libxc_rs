//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 645/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk645<F: Float>(t1979: F, t3625: F, t730: F, t154: F, t2089: F, t3542: F, t3515: F, t742: F, t1123: F) -> (F, F, F, F, F) {
    let t3626 = t3625 * t1979;
    let t3628 = F::new(0.17315859105681463759e2) * t730 * t3626;
    let t3631 = t154 * t2089 * t3542;
    let t3635 = t154 * t742 * t3515;
    let t3638 = t1123 * t1123;
    (t3626, t3628, t3631, t3635, t3638)
}
