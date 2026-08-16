//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1222/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1222<F: Float>(t20849: F, t237: F, t20904: F, t20913: F, t20916: F, t20921: F, t20924: F, t21270: F, t21273: F, t21275: F, t21277: F, t21281: F) -> (F, F) {
    let t21283 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t20849;
    let t21284 = t20904 - t21270 + t21273 - t21275 - t20913 + t20916 + t20921 + t20924 - t21277 + t21281 + t21283;
    (t21283, t21284)
}
