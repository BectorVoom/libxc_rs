//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 763/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk763<F: Float>(t2053: F, t566: F, t1629: F, t2819: F, t2781: F, t4830: F, t4823: F, t648: F) -> (F, F, F, F) {
    let t9639 = t566 * t2053;
    let t9642 = t1629 * t2819;
    let t9645 = t4830 * t2781;
    let t9648 = t4823 * t648;
    (t9639, t9642, t9645, t9648)
}
