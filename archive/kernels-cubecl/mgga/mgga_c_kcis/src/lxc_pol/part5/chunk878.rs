//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 878/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk878<F: Float>(t2035: F, t2047: F, t556: F, t7257: F, t572: F, t1533: F, t555: F, t7202: F, t583: F, t578: F, t2051: F, t2062: F) -> (F, F, F, F, F, F, F) {
    let t7280 = t2035 * t2047;
    let t7282 = t556 * t7257;
    let t7283 = t572 * t7282;
    let t7284 = t1533 * t7283;
    let t7286 = t555 * t7202;
    let t7287 = t583 * t7286;
    let t7288 = t578 * t7287;
    let t7290 = t2051 * t2062;
    (t7280, t7282, t7283, t7284, t7287, t7288, t7290)
}
