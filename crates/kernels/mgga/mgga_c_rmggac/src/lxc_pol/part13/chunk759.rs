//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 759/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk759<F: Float>(t1451: F, t1979: F, t1982: F, t201: F, t446: F, t2283: F, t7921: F, t2185: F, t8675: F, t1997: F, t1986: F, t5277: F, t675: F, t1540: F, t880: F, t2141: F) -> (F, F, F, F, F) {
    let t38963 = t446 * t1451 * t201 * t1979 * t1982;
    let t38965 = t7921 * t2283;
    let t38967 = t8675 * t2185;
    let t38968 = t38967 * t1997;
    let t38971 = t675 * t1986 * t5277;
    let t38973 = t1540 * t880;
    let t38974 = t38973 * t2141;
    (t38963, t38965, t38968, t38971, t38974)
}
