//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 910/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk910<F: Float>(t20634: F, t6001: F, t14265: F, t19067: F, t14213: F, t8010: F, t1340: F, t1411: F, t2059: F, t5967: F, t3797: F, t3796: F, t3482: F, t220: F, t2231: F, t19740: F) -> (F, F, F, F, F, F, F, F) {
    let t25383 = t20634 * t6001;
    let t25384 = t14265 * t25383;
    let t25385 = t19067 * t25384;
    let t25387 = t14213 * t8010;
    let t25388 = t1340 * t25387;
    let t25389 = t1411 * t25388;
    let t25391 = t2059 * t5967;
    let t25392 = t3797 * t25391;
    let t25393 = t3796 * t25392;
    let t25394 = t3482 * t25393;
    let t25396 = t220 * t2231;
    let t25397 = t3797 * t25396;
    let t25398 = t3796 * t25397;
    let t25399 = t19740 * t25398;
    (t25383, t25385, t25387, t25389, t25392, t25394, t25397, t25399)
}
