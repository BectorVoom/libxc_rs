//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1357/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1357<F: Float>(t2059: F, t33357: F, t4158: F, t6183: F, t13440: F, t442: F, t3922: F, t12951: F, t1328: F, t110605: F, t1163: F, t33398: F, t1411: F, t1440: F, t33608: F, t6376: F) -> (F, F, F, F, F) {
    let t113815 = t6183 * t33357 * t2059 * t4158;
    let t113818 = t13440 * t442;
    let t113821 = t6183 * t113818 * t2059 * t3922;
    let t113832 = t1328 * t12951;
    let t113846 = t110605 * t33398 * t1163;
    let t113851 = t1411 * t33608 * t6376 * t1440;
    (t113815, t113821, t113832, t113846, t113851)
}
