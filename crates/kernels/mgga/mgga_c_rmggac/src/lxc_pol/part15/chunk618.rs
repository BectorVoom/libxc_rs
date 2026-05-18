//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 618/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk618<F: Float>(t2186: F, t2310: F, t2289: F, t2286: F, t2283: F, t1614: F, t36: F, t262: F, t2103: F, t1587: F) -> (F, F, F, F, F, F, F, F) {
    let t8692 = t2186 * t2310;
    let t8694 = t2186 * t2289;
    let t8696 = t2186 * t2286;
    let t8698 = t2186 * t2283;
    let t8700 = t36 * t1614;
    let t8701 = t262 * t8700;
    let t8702 = t2103 * t8701;
    let t8704 = t36 * t1587;
    (t8692, t8694, t8696, t8698, t8700, t8701, t8702, t8704)
}
