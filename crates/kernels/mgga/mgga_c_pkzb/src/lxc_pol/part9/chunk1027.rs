//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1027/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1027<F: Float>(t2099: F, t3237: F, t3235: F, t2411: F, t3026: F, t824: F, t758: F, t2185: F, t3236: F, t1167: F, t6404: F, t2226: F) -> (F, F, F, F, F, F, F, F) {
    let t8406 = t2099 * t3237;
    let t8408 = F::cast_from(0.17149607247227894789e-2_f64) * t3235 * t8406;
    let t8409 = t2411 * t3026;
    let t8410 = t8409 * t824;
    let t8411 = t758 * t8410;
    let t8414 = t3236 * t2185;
    let t8415 = t758 * t8414;
    let t8418 = t6404 * t1167;
    let t8419 = t8418 * t2226;
    (t8408, t8409, t8410, t8411, t8414, t8415, t8418, t8419)
}
