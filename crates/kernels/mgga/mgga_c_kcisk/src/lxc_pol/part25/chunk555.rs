//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 555/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk555<F: Float>(t604: F, t5031: F, t5032: F, t1310: F, t4794: F) -> (F, F, F) {
    let t659 = 0.0 < t604;
    let t5033 = t5031 * t5032;
    let t5034 = t1310 * t5033;
    let t5038 = piecewise3(t659, t4794, -t4794);
    (t5033, t5034, t5038)
}
