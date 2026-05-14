//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1125/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1125<F: Float>(t2803: F, t33176: F, t11966: F, t5437: F, t2805: F, t1586: F) -> (F, F, F) {
    let t33177 = t33176 * t2803;
    let t33178 = t11966 * t5437;
    let t33179 = t2805 * t33178;
    let t33180 = t1586 * t33179;
    (t33177, t33179, t33180)
}
