//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 988/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk988<F: Float>(t3508: F, t5968: F, t1411: F, t19055: F, t3488: F, t13306: F, t6226: F, t1163: F, t6376: F, t3796: F, t3482: F, t13959: F, t2059: F, t3732: F, t3797: F, t3512: F, t6007: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19987 = t3508 * t5968;
    let t19988 = t1411 * t19987;
    let t19994 = t19055 * t3488;
    let t19996 = t13306 * t6226;
    let t19998 = t6376 * t1163;
    let t19999 = t3796 * t19998;
    let t20000 = t3482 * t19999;
    let t20002 = t13959 * t6226;
    let t20005 = t3797 * t2059 * t3732;
    let t20006 = t3796 * t20005;
    let t20007 = t3482 * t20006;
    let t20009 = t3512 * t6007;
    (t19988, t19994, t19996, t19998, t20000, t20002, t20005, t20007, t20009)
}
