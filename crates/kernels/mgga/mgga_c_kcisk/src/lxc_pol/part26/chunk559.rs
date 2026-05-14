//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 559/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk559<F: Float>(t1197: F, t240: F, t1568: F, t1576: F, t3806: F, t1607: F, t1610: F) -> (F, F, F, F) {
    let t4486 = t240 * t1197;
    let t4505 = t1568 * t1576;
    let t4519 = 0.38691203703703703703e-3 * t3806;
    let t4530 = t1607 * t1610;
    (t4486, t4505, t4519, t4530)
}
