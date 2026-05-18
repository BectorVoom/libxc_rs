//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 651/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk651<F: Float>(t1979: F, t1982: F, t8688: F, t2186: F, t2310: F, t2289: F, t2286: F, t2283: F, t1614: F, t36: F) -> (F, F, F, F, F, F) {
    let t8690 = t8688 * t1979 * t1982;
    let t8692 = t2186 * t2310;
    let t8694 = t2186 * t2289;
    let t8696 = t2186 * t2286;
    let t8698 = t2186 * t2283;
    let t8700 = t36 * t1614;
    (t8690, t8692, t8694, t8696, t8698, t8700)
}
