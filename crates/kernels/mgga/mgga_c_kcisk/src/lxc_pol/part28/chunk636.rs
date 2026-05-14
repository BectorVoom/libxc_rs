//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 636/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk636<F: Float>(t604: F, t2464: F, t5030: F, t1785: F, t7261: F, t6884: F) -> (F, F, F, F) {
    let t659 = 0.0 < t604;
    let t7262 = t5030 * t2464;
    let t7263 = t7262 * t1785;
    let t7264 = t7261 * t7263;
    let t7268 = piecewise3(t659, t6884, -t6884);
    (t7262, t7263, t7264, t7268)
}
