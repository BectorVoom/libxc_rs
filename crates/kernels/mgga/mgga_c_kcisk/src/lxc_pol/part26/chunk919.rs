//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 919/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk919<F: Float>(t1375: F, t25432: F, t25437: F, t457: F, t1186: F, t25446: F, t12829: F, t7706: F, t1056: F) -> (F, F, F, F) {
    let t25455 = t1375 * t25432;
    let t25458 = t457 * t25437;
    let t25461 = t1186 * t25446;
    let t25464 = t12829 * t7706;
    let t25465 = t25464 * t1056;
    (t25455, t25458, t25461, t25465)
}
