//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1255/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1255<F: Float>(t14030: F, t1943: F, t3974: F, t3977: F, t4039: F, t6209: F, t1529: F, t2425: F, t3437: F, t6875: F, t14190: F, t14193: F, t14235: F, t14238: F, t14242: F, t14245: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18652 = 32.0 / 27.0 * t3974 * t14030 * t1943 * t3977;
    let t18654 = 8.0 / 15.0 * t6209 * t4039;
    let t18655 = t2425 * t1529;
    let t18656 = 4.0 / 135.0 * t18655;
    let t18658 = 8.0 / 15.0 * t6875 * t3437;
    let t18659 = 32.0 / 135.0 * t14190;
    let t18660 = 32.0 / 27.0 * t14193;
    let t18661 = 32.0 / 135.0 * t14235;
    let t18662 = 32.0 / 27.0 * t14238;
    let t18663 = 128.0 / 243.0 * t14242;
    let t18664 = 16.0 / 81.0 * t14245;
    (t18652, t18654, t18656, t18658, t18659, t18660, t18661, t18662, t18663, t18664)
}
