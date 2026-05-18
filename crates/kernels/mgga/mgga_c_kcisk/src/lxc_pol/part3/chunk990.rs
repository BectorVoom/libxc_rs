//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 990/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk990<F: Float>(t14578: F, t1504: F, t4214: F, t469: F, t4205: F, t1513: F, t4301: F, t1501: F, t4182: F, t1488: F, t4312: F, t1487: F) -> (F, F, F, F, F) {
    let t14579 = t1504 * t14578;
    let t14581 = t4214 * t469;
    let t14582 = t14581 * t4205;
    let t14584 = t4301 * t1513;
    let t14586 = t1501 * t4182;
    let t14588 = t4312 * t1488;
    let t14589 = t1487 * t14588;
    (t14579, t14582, t14584, t14586, t14589)
}
