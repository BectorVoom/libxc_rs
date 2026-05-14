//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 337/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk337<F: Float>(t2097: F, t334: F, t1205: F, t1208: F, t2077: F, t2084: F, t2087: F, t2090: F) -> (F, F) {
    let t2098 = t2097 * t334;
    let t2105 = 0.258925e1 * t2084 - t1205 - 0.301925e0 * t2077 + 0.16504875e0 * t2087 - t1208 - 0.82785e-1 * t2090;
    (t2098, t2105)
}
