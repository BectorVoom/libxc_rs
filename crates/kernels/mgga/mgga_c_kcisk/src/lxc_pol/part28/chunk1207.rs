//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1207/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1207<F: Float>(t2029: F, t5507: F, t24991: F, t7261: F, t2028: F, t2642: F, t33197: F) -> (F, F, F, F, F, F) {
    let t34422 = t5507 * t2029;
    let t34423 = t34422 * t24991;
    let t34424 = t7261 * t34423;
    let t34427 = t2642 * t2028;
    let t34428 = t33197 * t34427;
    let t34429 = t7261 * t34428;
    (t34422, t34423, t34424, t34427, t34428, t34429)
}
