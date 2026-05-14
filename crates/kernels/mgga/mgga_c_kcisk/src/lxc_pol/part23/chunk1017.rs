//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1017/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1017<F: Float>(t1242: F, t20392: F, t19508: F, t3118: F, t353: F, t398: F, t5814: F, t1248: F, t5601: F, t1235: F, t13669: F, t2119: F, t4038: F, t13632: F, t4046: F, t6043: F) -> (F, F, F, F, F, F, F) {
    let t20393 = t1242 * t20392;
    let t20402 = t353 * t3118 * t19508;
    let t20404 = t5814 * t398;
    let t20406 = t1248 * t20404 * t5601;
    let t20409 = t1235 * t20392;
    let t20411 = t13669 * t2119;
    let t20412 = t20411 * t4038;
    let t20414 = t13632 * t2119;
    let t20415 = t20414 * t4038;
    let t20417 = t6043 * t4046;
    (t20393, t20402, t20406, t20409, t20412, t20415, t20417)
}
